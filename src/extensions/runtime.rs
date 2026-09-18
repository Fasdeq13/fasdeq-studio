use std::sync::{Arc, Mutex};
use wasmtime::*;

#[derive(Clone, Debug)]
pub enum HostCall {
    ShowMessage(String),
    RegisterCommand(String, String),
    RegisterPanel(String, String),
    NetworkRequest {
        url: String,
        method: String,
        body: String,
    },
    ReadActiveBuffer,
    WriteActiveBuffer(String),
}

#[derive(Clone, Debug)]
pub enum HostResponse {
    Ok,
    Text(String),
    Error(String),
}

pub struct ExtensionState {
    pub extension_id: String,
    pub allow_network: bool,
    pub allow_filesystem: bool,
    pub pending_calls: Arc<Mutex<Vec<HostCall>>>,
    pub last_buffer_content: Arc<Mutex<String>>,
}

pub struct ExtensionRuntime {
    engine: Engine,
}

impl ExtensionRuntime {
    pub fn new() -> anyhow::Result<Self> {
        let mut config = Config::new();
        config.wasm_component_model(false);
        config.consume_fuel(true);
        let engine = Engine::new(&config)?;
        Ok(Self { engine })
    }

    pub fn load_module(&self, wasm_bytes: &[u8]) -> anyhow::Result<Module> {
        Module::new(&self.engine, wasm_bytes)
    }

    pub fn instantiate(
        &self,
        module: &Module,
        extension_id: String,
        allow_network: bool,
        allow_filesystem: bool,
    ) -> anyhow::Result<(Store<ExtensionState>, Instance)> {
        let state = ExtensionState {
            extension_id,
            allow_network,
            allow_filesystem,
            pending_calls: Arc::new(Mutex::new(Vec::new())),
            last_buffer_content: Arc::new(Mutex::new(String::new())),
        };

        let mut store = Store::new(&self.engine, state);
        store.set_fuel(10_000_000_000)?;

        let mut linker = Linker::new(&self.engine);
        register_host_functions(&mut linker)?;

        let instance = linker.instantiate(&mut store, module)?;
        Ok((store, instance))
    }

    pub fn call_activate(
        &self,
        store: &mut Store<ExtensionState>,
        instance: &Instance,
    ) -> anyhow::Result<()> {
        if let Ok(activate) = instance.get_typed_func::<(), ()>(&mut *store, "activate") {
            activate.call(&mut *store, ())?;
        }
        Ok(())
    }

    pub fn call_command(
        &self,
        store: &mut Store<ExtensionState>,
        instance: &Instance,
        command_id: &str,
    ) -> anyhow::Result<()> {
        if let Some(memory) = instance.get_memory(&mut *store, "memory") {
            let ptr = write_string_to_memory(store, &memory, command_id)?;
            if let Ok(execute) = instance.get_typed_func::<(i32, i32), ()>(&mut *store, "execute_command") {
                execute.call(&mut *store, (ptr.0, ptr.1))?;
            }
        }
        Ok(())
    }
}

fn write_string_to_memory(
    store: &mut Store<ExtensionState>,
    memory: &Memory,
    text: &str,
) -> anyhow::Result<(i32, i32)> {
    let bytes = text.as_bytes();
    let current_size = memory.data_size(&mut *store);
    let needed = bytes.len();
    if needed > current_size {
        let additional_pages = ((needed - current_size) / 65536) as u64 + 1;
        memory.grow(&mut *store, additional_pages)?;
    }
    let offset = current_size.saturating_sub(bytes.len()).max(1024);
    memory.write(&mut *store, offset, bytes)?;
    Ok((offset as i32, bytes.len() as i32))
}

fn register_host_functions(linker: &mut Linker<ExtensionState>) -> anyhow::Result<()> {
    linker.func_wrap(
        "fasdeq",
        "show_message",
        |mut caller: Caller<'_, ExtensionState>, ptr: i32, len: i32| {
            let text = read_string_from_caller(&mut caller, ptr, len);
            let state = caller.data();
            state
                .pending_calls
                .lock()
                .unwrap()
                .push(HostCall::ShowMessage(text));
        },
    )?;

    linker.func_wrap(
        "fasdeq",
        "register_command",
        |mut caller: Caller<'_, ExtensionState>, id_ptr: i32, id_len: i32, title_ptr: i32, title_len: i32| {
            let id = read_string_from_caller(&mut caller, id_ptr, id_len);
            let title = read_string_from_caller(&mut caller, title_ptr, title_len);
            let state = caller.data();
            state
                .pending_calls
                .lock()
                .unwrap()
                .push(HostCall::RegisterCommand(id, title));
        },
    )?;

    linker.func_wrap(
        "fasdeq",
        "register_panel",
        |mut caller: Caller<'_, ExtensionState>, id_ptr: i32, id_len: i32, title_ptr: i32, title_len: i32| {
            let id = read_string_from_caller(&mut caller, id_ptr, id_len);
            let title = read_string_from_caller(&mut caller, title_ptr, title_len);
            let state = caller.data();
            state
                .pending_calls
                .lock()
                .unwrap()
                .push(HostCall::RegisterPanel(id, title));
        },
    )?;

    linker.func_wrap(
        "fasdeq",
        "network_request",
        |mut caller: Caller<'_, ExtensionState>,
         url_ptr: i32,
         url_len: i32,
         method_ptr: i32,
         method_len: i32,
         body_ptr: i32,
         body_len: i32| {
            let url = read_string_from_caller(&mut caller, url_ptr, url_len);
            let method = read_string_from_caller(&mut caller, method_ptr, method_len);
            let body = read_string_from_caller(&mut caller, body_ptr, body_len);
            let allowed = caller.data().allow_network;
            if allowed {
                caller
                    .data()
                    .pending_calls
                    .lock()
                    .unwrap()
                    .push(HostCall::NetworkRequest { url, method, body });
            }
        },
    )?;

    linker.func_wrap(
        "fasdeq",
        "read_active_buffer",
        |caller: Caller<'_, ExtensionState>| {
            caller
                .data()
                .pending_calls
                .lock()
                .unwrap()
                .push(HostCall::ReadActiveBuffer);
        },
    )?;

    linker.func_wrap(
        "fasdeq",
        "write_active_buffer",
        |mut caller: Caller<'_, ExtensionState>, ptr: i32, len: i32| {
            let text = read_string_from_caller(&mut caller, ptr, len);
            caller
                .data()
                .pending_calls
                .lock()
                .unwrap()
                .push(HostCall::WriteActiveBuffer(text));
        },
    )?;

    Ok(())
}

fn read_string_from_caller(caller: &mut Caller<'_, ExtensionState>, ptr: i32, len: i32) -> String {
    let Some(memory) = caller.get_export("memory").and_then(|e| e.into_memory()) else {
        return String::new();
    };
    let mut buffer = vec![0u8; len.max(0) as usize];
    if memory.read(caller, ptr as usize, &mut buffer).is_ok() {
        String::from_utf8_lossy(&buffer).to_string()
    } else {
        String::new()
    }
}

pub fn drain_pending_calls(state: &ExtensionState) -> Vec<HostCall> {
    let mut guard = state.pending_calls.lock().unwrap();
    std::mem::take(&mut *guard)
}

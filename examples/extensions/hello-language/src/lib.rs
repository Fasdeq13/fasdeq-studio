#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern "C" {
    fn show_message(ptr: i32, len: i32);
    fn register_command(id_ptr: i32, id_len: i32, title_ptr: i32, title_len: i32);
}

static mut SCRATCH: [u8; 1024] = [0; 1024];

fn write_scratch(text: &str) -> (i32, i32) {
    let bytes = text.as_bytes();
    let len = bytes.len().min(1024);
    unsafe {
        SCRATCH[..len].copy_from_slice(&bytes[..len]);
        (SCRATCH.as_ptr() as i32, len as i32)
    }
}

#[no_mangle]
pub extern "C" fn activate() {
    let (id_ptr, id_len) = write_scratch("zig.formatFile");
    let (title_ptr, title_len) = write_scratch("Zig: Format Current File");
    unsafe {
        register_command(id_ptr, id_len, title_ptr, title_len);
        let (msg_ptr, msg_len) = write_scratch("Zig Language Support extension activated");
        show_message(msg_ptr, msg_len);
    }
}

#[no_mangle]
pub extern "C" fn execute_command(_ptr: i32, _len: i32) {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern "C" {
    fn show_message(ptr: i32, len: i32);
    fn register_command(id_ptr: i32, id_len: i32, title_ptr: i32, title_len: i32);
    fn register_panel(id_ptr: i32, id_len: i32, title_ptr: i32, title_len: i32);
    fn network_request(
        url_ptr: i32,
        url_len: i32,
        method_ptr: i32,
        method_len: i32,
        body_ptr: i32,
        body_len: i32,
    );
    fn read_active_buffer();
    fn write_active_buffer(ptr: i32, len: i32);
}

static mut SCRATCH: [u8; 4096] = [0; 4096];

fn write_scratch(text: &str) -> (i32, i32) {
    let bytes = text.as_bytes();
    let len = bytes.len().min(4096);
    unsafe {
        SCRATCH[..len].copy_from_slice(&bytes[..len]);
        (SCRATCH.as_ptr() as i32, len as i32)
    }
}

#[no_mangle]
pub extern "C" fn activate() {
    let (id_ptr, id_len) = write_scratch("claude.explainSelection");
    let (title_ptr, title_len) = write_scratch("Claude: Explain Selected Code");
    unsafe {
        register_command(id_ptr, id_len, title_ptr, title_len);
        register_panel(id_ptr, id_len, title_ptr, title_len);
        let (msg_ptr, msg_len) = write_scratch("Claude Assistant extension activated");
        show_message(msg_ptr, msg_len);
    }
}

#[no_mangle]
pub extern "C" fn execute_command(_ptr: i32, _len: i32) {
    unsafe {
        read_active_buffer();
        let (url_ptr, url_len) = write_scratch("https://api.anthropic.com/v1/messages");
        let (method_ptr, method_len) = write_scratch("POST");
        let (body_ptr, body_len) = write_scratch("{}");
        network_request(url_ptr, url_len, method_ptr, method_len, body_ptr, body_len);
        let (out_ptr, out_len) = write_scratch("Requested explanation from Claude");
        write_active_buffer(out_ptr, out_len);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

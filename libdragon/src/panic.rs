use crate::*;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let (file, line) = match info.location() {
        Some(location) => {
            (CString::new(location.file()).unwrap(), location.line())
        },
        _ => (CString::new("<unknown>").unwrap(), 0)
    };

    let msg = if let Some(args) = info.message() {
        CString::new(format!("{}", args).as_str()).unwrap()
    } else {
        CString::new("<unknown>").unwrap()
    };

    let failedexpr = CString::new("<rust abort>").unwrap();
    let fmt = CString::new("%s").unwrap();

    unsafe {
        libdragon_sys::debug_assert_func_f(file.as_ptr(), line as i32, core::ptr::null(), failedexpr.as_ptr(), fmt.as_ptr(), msg.as_ptr());
    }
}



use std::fs::OpenOptions;
use std::io::Write;
use libc::{c_char, c_void};
use std::ptr::{null, null_mut};

extern "C" fn write_cb(_: *mut c_void, message: *const c_char) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("metrics/rc.txt")
        .unwrap();

    if let Err(e) = write!(file, "{}", String::from_utf8_lossy(unsafe {
        std::ffi::CStr::from_ptr(message as *const i8).to_bytes()
    })) {
       eprintln!("write stats failed: {}", e); 
    }
}

pub fn mem_print() {
    unsafe { 
        jemalloc_sys::malloc_stats_print(
            Some(write_cb), 
            null_mut(), 
            null()
        ) 
    }
}
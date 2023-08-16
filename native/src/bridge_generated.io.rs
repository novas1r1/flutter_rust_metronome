use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_set_bpm(port_: i64, bpm: u32) {
    wire_set_bpm_impl(port_, bpm)
}

#[no_mangle]
pub extern "C" fn wire_play(port_: i64) {
    wire_play_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_stop(port_: i64) {
    wire_stop_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_init_logger(port_: i64) {
    wire_init_logger_impl(port_)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

// Section: wire structs

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}

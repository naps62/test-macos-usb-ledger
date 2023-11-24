#[cfg(feature = "linux-static-rusb")]
extern crate rusb;

extern crate libc;

mod ffi;

fn main() {
    unsafe {
        ffi::hid_init();
    }
}

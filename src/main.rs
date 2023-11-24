#[cfg(feature = "linux-static-rusb")]
extern crate rusb;

extern crate libc;

use libc::c_int;

#[allow(dead_code)]
extern "C" {
    pub fn hid_init() -> c_int;
}

fn main() {
    unsafe {
        hid_init();
    }
}

mod hidapi_rusb;

fn main() {
    unsafe {
        hidapi_rusb::ffi::hid_init();
    }
}

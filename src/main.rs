mod hidapi_rusb;

fn main() {
    hidapi_rusb::HidApiLock::acquire().unwrap();
}

mod hidapi_rusb;

fn main() {
    hidapi_rusb::HidApi::new().unwrap();
}

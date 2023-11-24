#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hidapi_rusb::HidApi::new();

    Ok(())
}

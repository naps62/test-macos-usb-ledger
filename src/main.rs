// use crate::{
//     common::{APDUAnswer, APDUCommand},
//     errors::LedgerError,
//     transports::hid,

// };
mod hid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hid::TransportNativeHID::new();

    Ok(())
}

use coins_ledger::transports::LedgerAsync;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let ledger = ethers_signers::Ledger::new(ethers_signers::HDPath::LedgerLive(0), 1).await;
    let transport = coins_ledger::transports::Ledger::init()
        .await
        .expect("can't init ledger");

    Ok(())
}

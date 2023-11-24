#[tokio::main]
async fn main() {
    let ledger = ethers_signers::Ledger::new(ethers_signers::HDPath::LedgerLive(0), 1).await;

    dbg!(&ledger);
}

#[tokio::main]
async fn main() {
    let ledger = ethers::signers::Ledger::new(ethers::signers::HDPath::LedgerLive(0), 1).await;

    dbg!(&ledger);

    println!("Hello, world!");
}

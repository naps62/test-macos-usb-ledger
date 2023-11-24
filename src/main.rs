use coins_ledger::transports::LedgerAsync;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    coins_ledger::transports::Ledger::init()
        .await
        .expect("can't init ledger");

    Ok(())
}

// Import crates
use anyhow::Result;
use ethers::{prelude::*};
use std::convert::TryFrom;

pub async fn transfers() -> Result<()> {

    // Connect to the ethereum network with ganache-cli
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    // Get accounts
    let accounts = provider.get_accounts().await?;
    let from = accounts[0];
    let to = accounts[1];
    println!("from: {:?}, to:{:?}", from, to);

    // Make transaction requests from accounts.
    let tx = TransactionRequest::new().to(to).value(1000).from(from);

    // Get initial balance
    let balance_before = provider.get_balance(from, None).await?;

    //Make Transaction
    let tx = provider.send_transaction(tx, None).await?;


    println!("TX Hash: {:?}", tx);

    // Get transaction count
    let nonce1 = provider
        .get_transaction_count(from, Some(BlockNumber::Latest.into()))
        .await?;

    let nonce2 = provider
        .get_transaction_count(from, Some(BlockNumber::Number(0.into()).into()))
        .await?;

    assert!(nonce2 < nonce1);

    //Get final balance
    let balance_after = provider.get_balance(from, None).await?;
    assert!(balance_after < balance_before);

    println!("Balance before {}", balance_before);
    println!("Balance after {}", balance_after);

    Ok(())
}

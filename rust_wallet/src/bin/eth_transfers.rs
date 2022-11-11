// Import crates
use anyhow::Result;
use ethers::{prelude::*};
use std::convert::TryFrom;
use std::thread::sleep;
use std::time::Duration;
use ethers_core::rand::thread_rng;
use ethers_signers::{LocalWallet, Signer};


pub async fn transfers() -> Result<()> {

    // Connect to the ethereum network with ganache-cli
    let provider = Provider::<Http>::try_from("http://127.0.0.1:8545")?;
    // Get accounts
    let accounts = provider.get_accounts().await?;
    let from = accounts[0];
    let to = accounts[1];
    // println!("from: {:?} to: {:?}", from, to);

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

pub async fn signs() -> Result<()> {
    //let wallet = LocalWallet::new(&mut rand::thread_rng()); remove rand since it's already declared
    let wallet = LocalWallet::new(&mut thread_rng());

    let message = "Some data";

    // sign a message
    let signature = wallet.sign_message(message).await?;
    println!("Produced signature {}", signature);

    // verify the signature
    signature.verify(message, wallet.address()).unwrap();

    println!("Verified signature produced by {:?}!", wallet.address());

    Ok(())
}

// https://hannydevelop.hashnode.dev/building-defi-with-rust-and-ethereum-providers-and-signers-ckppk54ic08fwwhs1edi7h8h1

#[tokio::main]
async fn main() {
    transfers().await.ok();
    sleep(Duration::from_secs(10));

    //signs().await.ok();
}

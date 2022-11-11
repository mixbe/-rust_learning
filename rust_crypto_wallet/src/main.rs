use std::env;
use std::str::FromStr;
use anyhow::Result;

mod eth_wallet;
mod utils;

use dotenv;
use web3::types::Address;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    // let (secret_key, pub_key) = eth_wallet::generate_keypair();
    // println!("secret_key: {}", secret_key.display_secret());
    // println!("pub_key: {}", pub_key.to_string());
    //
    // let address = eth_wallet::public_key_address(&pub_key);
    // println!("address: {:?}", address);
    //
    // let wallet = eth_wallet::Wallet::new(&secret_key, &pub_key);
    // println!("wallet: {:?}", wallet);
    //
    // wallet.save_to_file("crypto_wallet.json")?;

    let wallet_file_path = "crypto_wallet.json";
    let loaded_wallet = eth_wallet::Wallet::load_wallet_file(wallet_file_path)?;


    let endpoint = env::var("END_POINT")?;
    let web3_con = loaded_wallet.establish_web3_connection(&endpoint).await?;
    let block_number = web3_con.eth().block_number().await?;
    println!("block_number: {:?}", block_number);

    let balance = loaded_wallet.get_balance_in_eth(&web3_con).await?;
    println!("balance: {:?}", balance);

    let transaction = eth_wallet::create_eth_transaction(
        Address::from_str("0xa80EBc3d80a9c5Bb599f7D3150fB0eD32178C97c")?, 10f64,
    );

    let transaction_hash = eth_wallet::sign_and_send(&web3_con, transaction, &loaded_wallet.get_secret_key()?).await?;
    println!("transaction: {:?}", transaction_hash);


    Ok(())
}

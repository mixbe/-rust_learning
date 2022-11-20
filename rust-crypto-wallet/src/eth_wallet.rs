use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use web3::{
    transports,
    types::{Address, U256},
    Web3,
};
use anyhow::Result;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use secp256k1::rand::{rngs, SeedableRng};
use web3::transports::WebSocket;
use crate::utils;
use web3::signing::keccak256;
use web3::types::{H256, TransactionParameters};


pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let mut rng = rngs::JitterRng::new_with_timer(utils::get_nstime);
    secp.generate_keypair(&mut rng)
}


// don't need or want to take ownership of th object
pub fn public_key_address(pub_key: &PublicKey) -> Address {
    let public_key = pub_key.serialize_uncompressed();

    debug_assert_eq!(public_key[0], 0x04);

    let hash = keccak256(&public_key[1..]);
    Address::from_slice(&hash[12..])
}

// 创建交易
pub fn create_eth_transaction(to: Address, eth_value: f64) -> TransactionParameters {
    TransactionParameters {
        to: Some(to),
        value: utils::eth_to_wei(eth_value),
        ..Default::default()
    }
}

// 签名并发送交易
pub async fn sign_and_send(web3: &Web3<WebSocket>, transaction: TransactionParameters, secret_key: &SecretKey) -> Result<H256> {
    let signed = web3.accounts().sign_transaction(transaction, secret_key).await?;

    let transaction_result = web3.eth().send_raw_transaction(signed.raw_transaction).await?;
    Ok(transaction_result)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub secret_key: String,
    pub public_key: String,
    pub public_address: String,
}

impl Wallet {
    pub fn new(secret_key: &SecretKey, public_key: &PublicKey) -> Self {
        let address = public_key_address(&public_key);
        Wallet {
            secret_key: secret_key.display_secret().to_string(),
            public_key: public_key.to_string(),
            public_address: format!("{:?}", address),
        }
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;
        let buf_writer = BufWriter::new(file);
        serde_json::to_writer_pretty(buf_writer, self)?;
        Ok(())
    }

    pub fn load_wallet_file(file_path: &str) -> Result<Wallet> {
        let file = OpenOptions::new().read(true).open(file_path)?;
        let buf_reader = BufReader::new(file);
        let wallet: Wallet = serde_json::from_reader(buf_reader)?;
        Ok(wallet)
    }

    pub fn get_secret_key(&self) -> Result<SecretKey> {
        let secret_key = SecretKey::from_str(&self.secret_key)?;

        Ok(secret_key)
    }

    pub fn get_public_key(&self) -> Result<PublicKey> {
        let public_key = PublicKey::from_str(&self.public_key)?;
        Ok(public_key)
    }

    // connect eth endpoint
    pub async fn establish_web3_connection(&self, url: &str) -> Result<Web3<WebSocket>> {
        let transport = web3::transports::WebSocket::new(url).await?;
        Ok(web3::Web3::new(transport))
    }

    pub async fn get_balance(&self, web3_con: &Web3<WebSocket>) -> Result<U256> {
        let wallet_address = Address::from_str(&self.public_address)?;
        let balance = web3_con.eth().balance(wallet_address, None).await?;
        Ok(balance)
    }

    pub async fn get_balance_in_eth(&self, web3_con: &Web3<WebSocket>) -> Result<f64> {
        let wei_balance = self.get_balance(web3_con).await?;
        Ok(utils::wei_to_eth(wei_balance))
    }
}


#[cfg(test)]
mod tests {
    use anyhow::Result;
    use crate::eth_wallet::{generate_keypair, public_key_address, Wallet};

    #[test]
    fn test() -> Result<()> {
        let (secret_key, pub_key) = generate_keypair();
        println!("secret_key: {}", secret_key.display_secret());
        println!("pub_key: {}", pub_key.to_string());

        let address = public_key_address(&pub_key);
        println!("address: {:?}", address);

        let wallet = Wallet::new(&secret_key, &pub_key);
        println!("wallet: {:?}", wallet);

        wallet.save_to_file("crypto_wallet.json")?;

        Ok(())
    }

    #[test]
    fn test_load_wallet_file() -> Result<()> {
        let file_path = "crypto_wallet.json";
        let wallet = Wallet::load_wallet_file(file_path)?;
        println!("{:?}", wallet);

        Ok(())
    }

    #[tokio::test]
    async fn test_web3_connection() -> Result<()> {
        let url = "ws://localhost:8545";
        //
        // let web3_con = Wallet::establish_web3_connection(<url).await?;
        // let block_number = web3_con.eth().block_number().await?;
        // println!("block_number: {:?}", block_number);

        Ok(())
    }
}
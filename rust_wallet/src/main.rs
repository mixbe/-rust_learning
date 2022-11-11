mod transfer;

#[tokio::main]
async fn main() {
    transfer::transfers().await.ok();
}

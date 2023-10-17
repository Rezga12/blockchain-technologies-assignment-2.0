const PRIVATE_KEY: &str = "";

#[tokio::main]
async fn main() {
    send("<P2PKH testnet address>", 1000).await;
}

async fn send(receiver_address: &str, amount_satoshis: u64) {
    todo!("Just make this function work")
}

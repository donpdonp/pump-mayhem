use fast_websocket_client::WebSocket;
use rust_yaml;
use solana_client::transaction_executor;
use std::{fs, net::SocketAddr};
use tokio::time::{sleep, Duration};

fn load_config(filename: &str) -> rust_yaml::value::Value {
    let config_yaml = fs::read_to_string(filename).unwrap();
    let yaml = rust_yaml::Yaml::new();
    return yaml.load_str(&config_yaml).unwrap();
}

fn _pump(sock: SocketAddr) {
    let _te = transaction_executor::TransactionExecutor::new(sock);
}

struct PumpMsgSub {
    method: String,
}

fn main() {
    let config = load_config("config.yaml");
    let url_str = config.get_str("ws").unwrap().as_str().unwrap();
    let _ = iomain(url_str);
}

#[tokio::main(flavor = "current_thread")]
async fn iomain(url: &str) -> Result<(), fast_websocket_client::WebSocketClientError> {
    let ws = WebSocket::new(url).await?;
    println!("iomain {}", url);

    ws.on_open(|_| async move {
        println!("[OPEN] WebSocket connection opened.");
    })
    .await;
    let pm_sub = PumpMsgSub {
        method: "subscribeNewToken".to_owned(),
    };
    ws.await_shutdown().await?;
    Ok(())
}

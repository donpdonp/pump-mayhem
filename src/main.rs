use fast_websocket_client::WebSocket;
use rust_yaml;
use solana_client::transaction_executor;
use std::{fs, net::SocketAddr};
use tokio::sync::mpsc;

mod pumpdev;

fn load_config(filename: &str) -> rust_yaml::value::Value {
    let config_yaml = fs::read_to_string(filename).unwrap();
    let yaml = rust_yaml::Yaml::new();
    return yaml.load_str(&config_yaml).unwrap();
}

fn _pump(sock: SocketAddr) {
    let _te = transaction_executor::TransactionExecutor::new(sock);
}

fn main() {
    let config = load_config("config.yaml");
    let url_str = config.get_str("ws").unwrap().as_str().unwrap();
    let _ = iomain(url_str);
}

async fn process(msg: pumpdev::PumpMsg, tx: mpsc::UnboundedSender<pumpdev::PumpSend>) {
    match msg {
        pumpdev::PumpMsg::Connected(msg) => {
            println!("{:?}", msg)
        }
        pumpdev::PumpMsg::Subscribed(msg) => {
            println!("{:?}", msg)
        }
        pumpdev::PumpMsg::ConnectionStatus(m) => {
            println!("{:?}", m);
            if m.connected {
                let sub_msg = pumpdev::SubscribeNewToken {};
                tx.send(pumpdev::PumpSend::SubscribeNewToken(sub_msg))
                    .unwrap();
            }
        }
        pumpdev::PumpMsg::NewToken(msg) => {
            println!("{:?}", msg)
        }
        pumpdev::PumpMsg::NewPool(msg) => {
            println!("{:?}", msg)
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn iomain(url: &str) -> Result<(), fast_websocket_client::WebSocketClientError> {
    let ws = WebSocket::new(url).await?;
    println!("iomain {}", url);
    let (tx, mut rx) = mpsc::unbounded_channel::<pumpdev::PumpSend>();

    ws.on_open(|_| async move {
        println!("[OPEN] WebSocket connection opened.");
    })
    .await;

    ws.on_close(|event| async move {
        println!(
            "[CLOSE] code={}, reason={:?}, clean={}",
            event.code, event.reason, event.was_clean
        );
    })
    .await;

    ws.on_message(move |message| {
        let tx2 = tx.clone();
        async move {
            println!("[MESSAGE] {} ", message);
            let msg: pumpdev::PumpMsg = serde_json::from_str(&message).unwrap();
            process(msg, tx2).await;
        }
    })
    .await;

    while let Some(pmsg) = rx.recv().await {
        let json = serde_json::to_string(&pmsg).unwrap();
        println!("[SEND] {}", json);
        ws.send(&json).await.unwrap();
    }

    ws.on_error(|error| async move {
        eprintln!("[ERROR] {}", error);
    })
    .await;

    ws.await_shutdown().await?;
    Ok(())
}

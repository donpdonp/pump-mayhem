use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PumpMsg {
    Subscribed(Subscribed),
    Connected(Connected),
    ConnectionStatus(ConnectionStatus),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Connected {
    client_id: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatus {
    pub connected: bool,
}

#[derive(Deserialize, Debug)]
pub struct Subscribed {
    method: String,
}

#[derive(Serialize, Debug)]
#[serde(tag = "method", rename_all = "camelCase")]
pub enum PumpSend {
    SubscribeNewToken(SubscribeNewToken),
}

#[derive(Serialize, Debug)]
pub struct SubscribeNewToken {}

use serde::Deserialize;

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
    connected: bool,
}

#[derive(Deserialize, Debug)]
pub struct Subscribed {
    method: String,
}

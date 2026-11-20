use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(untagged, rename_all = "camelCase")]
pub enum PumpMsg {
    Subscribed(Subscribed),
    Connected(Connected),
    ConnectionStatus(ConnectionStatus),
    NewToken(NewToken),
    NewPool(NewPool),
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
#[serde(rename_all = "camelCase")]
pub struct Subscribed {
    method: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewToken {
    pub signature: String,
    pub mint: String,
    pub trader_public_key: String,
    pub tx_type: String,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub initial_buy: f64,
    pub initial_quote_amount: f64,
    pub quote_amount_raw: i64,
    pub bonding_curve_key: String,
    pub v_tokens_in_bonding_curve: i64,
    pub v_quote_in_bonding_curve: i64,
    pub token_program: String,
    pub is_mayhem_mode: bool,
    pub is_cashback_enabled: bool,
    pub is_holder_reward: bool,
    pub quote_mint: String,
    pub quote_token_decimals: i64,
    pub quote_amount: f64,
    pub market_cap_quote: f64,
    // pub v_sol_in_bonding_curve: i64,
    // pub sol_amount: f64,
    // pub market_cap_sol: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewPool {
    pub signature: String,
    pub mint: String,
    pub tx_type: String,
    pub pool: String,
    pub canonical_pool: String,
    pub is_canonical_pool: bool,
    pub creator: String,
    pub trader_public_key: String,
    pub base_mint: String,
    pub pair_quote_mint: String,
    pub source: String,
    pub pool_base_reserves: i64,
    pub pool_base_reserves_ui: f64,
    pub pool_quote_reserves: i64,
    pub pool_quote_reserves_ui: f64,
    pub quote_mint: String,
    pub quote_token_decimals: i32,
    pub quote_amount: f64,
    pub market_cap_quote: f64,
    pub sol_amount: f64,
    pub market_cap_sol: f64,
}

#[derive(Serialize, Debug)]
#[serde(tag = "method", rename_all = "camelCase")]
pub enum PumpSend {
    SubscribeNewToken(SubscribeNewToken),
}

#[derive(Serialize, Debug)]
pub struct SubscribeNewToken {}

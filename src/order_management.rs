use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub enum OrderType {
    #[serde(rename = "ORDER_TYPE_BUY")]
    OrderTypeBuy,
    #[serde(rename = "ORDER_TYPE_SELL")]
    OrderTypeSell,
}

#[derive(Serialize, Debug)]
pub struct TradeRequest {
    pub symbol: String,
    #[serde(rename = "actionType")]
    pub action_type: OrderType,
    pub volume: f32,
    #[serde(rename = "openPrice", skip_serializing_if = "Option::is_none")]
    pub open_price: Option<f32>,
    #[serde(rename = "takeProfit", skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<f32>,
    #[serde(rename = "stopLoss", skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct TradeResponse {
    #[serde(rename = "numericCode")]
    pub numeric_code: i32,
    #[serde(rename = "stringCode")]
    pub string_code: String,
    pub message: String,
    #[serde(rename = "orderId")]
    pub order_id: Option<String>,
    #[serde(rename = "positionId")]
    pub position_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Candle {
    pub symbol: String,
    pub timeframe: String,
    pub time: String,
    #[serde(rename = "brokerTime")]
    pub broker_time: String,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    #[serde(rename = "tickVolume")]
    pub tick_volume: u32,
    pub spread: f32,
}

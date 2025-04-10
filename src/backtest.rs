use crate::backend::*;
use crate::order_management::Candle;
use chrono::{DateTime, Utc};
use anyhow::{bail, Context, Ok, Result};

pub struct Backtest {
    starting_balance: f32,
    current_balance: f32,
    trades: Vec<Trade>,
    asset: Symbol,
}

pub struct Trade {
    entry_time: DateTime<Utc>,
    entry_price: f32,
    take_profit: f32,
    volume: f32, 
    profit_or_loss: f32
}

pub async fn load_historical_data(bot: TradingBot, asset: Symbol) -> Result<(Vec<Candle>, Vec<Candle>)> {
    let candles_4h = bot.get_historical_data(&asset, "4h", 1000).await?;
    let candles_15m = bot.get_historical_data(&asset, "15m", 1000).await?;
    Ok((candles_4h, candles_15m ))
}

impl Backtest {
    pub fn new(starting_balance: f32, asset: Symbol) -> Backtest {
        Backtest { starting_balance, current_balance: starting_balance, trades: Vec::new(), asset }
    }

}



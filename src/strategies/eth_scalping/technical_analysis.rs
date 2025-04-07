use crate::backend::*;
use crate::order_management::*;
use std::time::Duration;
use anyhow::{Ok, Result};

pub async fn scalp_long(bot: TradingBot, asset: Symbol) -> Result<()> {
    let currency = Symbol::match_str(asset)?;

    let candles = bot.get_historical_data(&currency, "4h", 186).await?;
    let mut lowest_close = &candles[0];

    for i in 1..candles.len() {
        if candles[i].close < lowest_close.close {
            lowest_close = &candles[i];
        }
    }

    loop {
        let first_15m_candle = loop {
            match bot.get_current_candle(&currency, "15m").await {
                Ok(candle) => break candle,
                Err(_) => continue,
            }
        };                                         
        if first_15m_candle.close <= lowest_close.close {
            tokio::time::sleep(tokio::time::Duration::from_secs(900)).await;
            let second_15m_candle = loop {
                match bot.get_current_candle(&currency, "15m").await {
                    Ok(candle) => break candle,
                    Err(_) => continue,
                }
            };
            //check that this candle is 15 ahead of the 1st one
            if second_15m_candle.close > first_15m_candle.close {
                let trade = TradeRequest{
                    symbol: currency, 
                    action_type: OrderTypeBuy,
                    volume: 5.0,
                    open_price: None,
                    take_profit: None,
                    stop_loss: None,
                };
                bot.open_trade(trade).await?;
            }
        }
    }
    Ok(())
}

pub async fn scalp_short() -> Result<()> {
    todo!();
}
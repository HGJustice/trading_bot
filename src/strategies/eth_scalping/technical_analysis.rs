use crate::backend::*;
use crate::order_management::*;
use anyhow::{Ok, Result};
use tokio::time::{Sleep, Duration};

pub async fn scalp_long(bot: TradingBot, asset: Symbol) -> Result<()> {
    let currency = Symbol::match_str(asset)?;

    let candles = bot.get_historical_data(currency, "4h", 186).await?;
    let mut lowest_close = &candles[0];

    for i in 1..&candles.len() {
        if candles[i].close < lowest_close.close {
            lowest_close = &candles[i];
        }
    }

    loop {
        let first_15m_candle = bot.get_current_candle(currency, "15m").await?; // this api calls bugs out, so gotta call couple time
                                                                                // till candle appears

        if first_15m_candle.close <= lowest_close.close {
            // sleep for the rest of the time till new 15 m, candle appears
            let second_15m_candle = bot.get_current_candle(currency, "15m").await?;
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
                break;
            }
        }
    }
    Ok(())
}
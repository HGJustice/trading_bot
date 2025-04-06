use anyhow::{Ok, Result};
use trading_bot::backend::*;
use trading_bot::order_management::*;

#[tokio::main]
async fn main() -> Result<()> {
    let bot = TradingBot::new().await?;
    println!("Bot initialized successfully");

    let candle = bot.get_current_candle(Symbol::ETH, "15m").await?;
    println!("Candle: {:#?}", candle);
    Ok(())
}

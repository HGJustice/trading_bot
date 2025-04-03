use anyhow::{Ok, Result};
use trading_bot::backend::*;
use trading_bot::order_management::*;

#[tokio::main]
async fn main() -> Result<()> {
    let bot = TradingBot::new().await?;
    println!("Bot initialized successfully");
   

    let candles = bot.get_historical_data(Symbol::ETH, String::from("1h")).await?;
    println!("Candles: {:#?}", candles);
    Ok(())
}

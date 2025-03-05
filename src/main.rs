use trading_bot::backend::*;
use anyhow::{ Ok, Result};

#[tokio::main]
async fn main() -> Result<()> {

    let bot = TradingBot::new().await?;
    println!("Authentication successful! youre in boss LINDDDA");

    let result = bot.check_balance().await?;
    println!("amount big ones brudda: {:?}", result);
    
    Ok(())
}
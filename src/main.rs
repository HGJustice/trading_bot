use anyhow::{Ok, Result};
use trading_bot::backend::*;
use trading_bot::order_management::*;

#[tokio::main]
async fn main() -> Result<()> {
    let bot = TradingBot::new().await?;
    println!("Bot initialized successfully");

    // let trade = TradeRequest {
    //     symbol: String::from("ETHUSD"),
    //     action_type: OrderType::OrderTypeBuy,
    //     volume: 5.0,   
    //     open_price: Option::None,
    //     stop_loss: Option::None,
    //     take_profit: Option::None,
    // };
   
    // let result = bot.open_trade(trade).await?;
    // println!("Trade: {:#?}", result);

    let candle = bot.get_current_candle(Symbol::ETH, "15m" ).await?;
    println!("Candle: {:#?}", candle);
    Ok(())
}

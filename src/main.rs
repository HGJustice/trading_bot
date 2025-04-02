use anyhow::{Ok, Result};
use trading_bot::backend::*;
use trading_bot::order_management::*;

#[tokio::main]
async fn main() -> Result<()> {
    let bot = TradingBot::new().await?;
    println!("Bot initialized successfully");
    // let buy_request = TradeRequest {
    //     symbol: "ETHUSD".to_string(),
    //     action_type: OrderType::OrderTypeBuy,
    //     volume: 1.0,       // 1 eth
    //     open_price: None,  // Market order
    //     stop_loss: None,   // Optional stop loss
    //     take_profit: None, // Optional take profit
    // };

    // let result = bot.execute_trade(buy_request).await?;
    let result = bot.close_trade("215212128".to_string()).await?;
    println!("Results: {:?}", result);

    Ok(())
}

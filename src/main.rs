use trading_bot::backend::*;
use trading_bot::order_management::*;
use anyhow::{ Ok, Result};

#[tokio::main]
async fn main() -> Result<()> {

    let bot = TradingBot::new().await?;
    let buy_request = TradeRequest {
        symbol: "ETHUSD".to_string(),
        action_type: OrderType::OrderTypeBuy,
        volume: 1.0,  // 1 eth 
        open_price: None,  // Market order
        stop_loss: None,  // Optional stop loss
        take_profit: None,  // Optional take profit
    };

    let result = bot.execute_trade(buy_request).await?;
    println!("Results: {:?}", result);
    Ok(())
}

use anyhow::{Ok, Result};
use trading_bot::backend::*;
use trading_bot::order_management::*;

#[tokio::main]
async fn main() -> Result<()> {
    let bot = TradingBot::new().await?;
    // let buy_request = TradeRequest {
    //     symbol: "ETHUSD".to_string(),
    //     action_type: OrderType::OrderTypeBuy,
    //     volume: 1.0,       // 1 eth
    //     open_price: None,  // Market order
    //     stop_loss: None,   // Optional stop loss
    //     take_profit: None, // Optional take profit
    // };

    // let result = bot.execute_trade(buy_request).await?;
    // println!("Results: {:?}", result);

    // let result = bot.close_trade("210031480".to_string()).await?;
    // println!("Results: {:?}", result);

    // let result = bot.get_asset_price(Symbol::BTC).await?;
    // println!("Results: {:?}", result);
    println!("Bot initialized successfully");

    // Test getting historical data
    let symbol = "BTCUSD"; // Or any symbol available in your account
    let timeframe = "1h";   // 1-hour candles
    let limit = 10;        // Get 10 candles

    let result = bot.get_historical_data(symbol, timeframe, limit).await?;
    println!("Results: {:?}", result);
    Ok(())
}

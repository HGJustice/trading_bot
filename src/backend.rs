use crate::order_management::*;
use anyhow::{ bail, Context, Ok, Result};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

pub enum Symbol {
    BTC,
    ETH,
}

impl Symbol {
    pub fn match_str(symbol: Symbol) -> Result<String>{
        match symbol {
            Symbol::BTC => {
                return Ok(String::from("BTCUSD"))
            }
            Symbol::ETH => {
              return Ok(String::from("ETHUSD"))
            }
        };
    }
}

pub struct TradingBot {
    client: Client,
    account_id: String,
    api_token: String,
}

impl TradingBot {
    pub async fn new() -> Result<Self> {
        dotenv().ok();
        let client = Client::new();

        let api_token =
            env::var("API_ACCESS_TOKEN").context("API_ACCESS_TOKEN not set in environment file")?;
        let account_id =
            env::var("ACCOUNT_ID").context("ACCOUNT_ID not set in environment file")?;

        Ok(Self {
            client,
            account_id,
            api_token,
        })
    }

    pub async fn check_balance(&self) -> Result<f64> {
        let url = format!(
            "{}/users/current/accounts/{}/account-information",
            env::var("API_URL").context("API_URL is not set in the environment file")?,
            self.account_id
        );

        let response = self
            .client
            .get(&url)
            .header("auth-token", &self.api_token)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            bail!("Failed to get account info: {}", error_text);
        }

        let account_info: serde_json::Value = response.json().await?;
        let balance = account_info["balance"].as_f64().unwrap_or(0.0);

        Ok(balance)
    }

    pub async fn get_asset_price(&self, asset: Symbol) -> Result<(f64, f64)> {

        let currency = Symbol::match_str(asset)?;

        let url = format!(
            "{}/users/current/accounts/{}/symbols/{}/current-price",
            env::var("API_URL").context("API_URL is not set in the environment file")?,
            &self.account_id, currency);

            let response = self
            .client
            .get(&url)
            .header("auth-token", &self.api_token)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            bail!("Failed to get price: {}", error_text);
        }

        let price_data: serde_json::Value = response.json().await?;
        let bid = price_data["bid"].as_f64().unwrap_or(0.0);
        let ask = price_data["ask"].as_f64().unwrap_or(0.0);
        
        Ok((bid, ask))
    }

    pub async fn get_historical_data(&self, asset: Symbol, timeframe: &str, limit: u32) -> Result<Vec<Candle>> {
        let currency = Symbol::match_str(asset)?;
    
        let url = format!("{}/users/current/accounts/{}/historical-market-data/symbols/{}/timeframes/{}/candles?limit={}", env::var("STATUS_API_URL").context("failed to load meta status api")?, &self.account_id, currency, timeframe, limit);
        
        let response = self.client
            .get(&url)
            .header("auth-token", &self.api_token)
            .send()
            .await?;
    
        if !response.status().is_success() {
            let error = response.text().await?;
            bail!("Failed getting historical data {}", error);
        }
    
        let candles: Vec<Candle> = response.json().await?;
        Ok(candles)
    }

    pub async fn get_current_candle(&self, asset: Symbol, timeframe: &str) -> Result<Candle>{
        let currency = Symbol::match_str(asset)?;

        let url = format!("{}/users/current/accounts/{}/symbols/{}/current-candles/{}?keepSubscription=false", env::var("API_URL").context("API URL not provided in env file")?, self.account_id, currency, timeframe);
        
        let response = self.client.get(&url).header("auth-token", &self.api_token).send().await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            bail!("Failed getting candle data {}", error);
        }

        let candle: Candle = response.json().await?;
        Ok(candle)
    }

    pub async fn open_trade(&self, trade: TradeRequest) -> Result<TradeResponse> {
        let url = format!(
            "{}/users/current/accounts/{}/trade",
            env::var("API_URL").context("API_URL is not set in the environment file")?,
            &self.account_id
        );

        let response = self
            .client
            .post(&url)
            .header("auth-token", &self.api_token)
            .json(&trade)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            bail!("Failed to execute trade: {}", error_text);
        }

        let trade_response: TradeResponse = response.json().await?;
        Ok(trade_response)
    }

    pub async fn close_trade(&self, position_id: &str) -> Result<TradeResponse> {
        let url = format!(
            "{}/users/current/accounts/{}/trade",
            env::var("API_URL").context("API_URL is not set in the environment file")?,
            &self.account_id
        );
        let close_request = serde_json::json!({
            "actionType": "POSITION_CLOSE_ID", 
            "positionId": position_id
        });
        
        let response = self.client.post(&url)
            .header("auth-token", &self.api_token)
            .json(&close_request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            bail!("Failed to close position: {}", error_text);
        }
        
        let trade_response: TradeResponse = response.json().await?;
        Ok(trade_response)
    }
}
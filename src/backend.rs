use crate::order_management::{TradeRequest, TradeResponse};
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
        todo!()
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

        let currency = match asset {
            Symbol::BTC => "BTCUSD".to_string(),
            Symbol::ETH => "ETHUSD".to_string(),
        };

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

    pub async fn get_historical_data(&self, symbol: Symbol, timeframe: String) -> Result<()> {
        let currency = match symbol {
            Symbol::BTC => "BTCUSD".to_string(),
            Symbol::ETH => "ETHUSD".to_string(),
        };


        let url = format!("{}/users/current/accounts/{}/historical-market-data/symbols/{}/timeframes/{}/candles", self.api_token, self.account_id, currency, timeframe);

        let responce = self.client.post(&url).header("auth-token", &self.api_token).send().await?;

        if !responce.status().is_success() {
            let error = responce.text().await?;
            bail!("Failed getting historical data {}", error);
        }


       Ok(())
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

    pub async fn close_trade(&self, position_id: String) -> Result<TradeResponse> {
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

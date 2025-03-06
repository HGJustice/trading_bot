use reqwest::{Client, header};
use anyhow::{anyhow, bail, Context, Ok, Result};
use dotenv::dotenv;
use std::env;

use crate::order_management::{TradeRequest, TradeResponse};

pub enum Symbol {
    ETH,
    BTC
}

pub struct TradingBot {
    client: Client,
    account_id: String,
    api_token: String, 
}

impl TradingBot {
    pub async fn new() -> Result<Self>{
        dotenv().ok();
        let client = Client::new();
        
        let api_token = env::var("API_ACCESS_TOKEN")
            .context("API_ACCESS_TOKEN not set in environment file")?;
        let account_id = env::var("ACCOUNT_ID")
            .context("ACCOUNT_ID not set in environment file")?;
        
        Ok(Self {
            client,
            account_id,
            api_token,
        })
    }

    pub async fn check_balance(&self,) -> Result<f64>{
        let url = format!("{}/users/current/accounts/{}/account-information", 
        env::var("API_URL").context("API_URL is not set in the environment file")?, self.account_id);

        let response = self.client.get(&url)
        .header("auth-token", &self.api_token)
        .send()
        .await?;

        if !response.status().is_success() {
        let error_text = response.text().await?;
        bail!("Failed to get account info: {}", error_text);
        }

        let account_info: serde_json::Value = response.json().await?;
        let balance = account_info["balance"].as_f64()
        .unwrap_or(0.0);

        Ok(balance)
    }

    pub async fn get_crypto_symbol(&self, crypto: Symbol) -> Result<String> {
        let url = format!("{}/users/current/accounts/{}/symbols", env::var("API_URL").context("API_URL is not set in the environment file")?, self.account_id);

        let response = self.client.get(url).header("auth-token", &self.api_token).send().await?;
        if !response.status().is_success() {
            let error_text = response.text().await?;
            bail!("Failed to get account info: {}", error_text);
        }

        let symbols: Vec<String> = response.json().await?;
        match crypto {
            Symbol::BTC => {
                let crypto_symbol = &symbols[14];
                Ok(crypto_symbol.to_string())
            }
            Symbol::ETH => {
                let crypto_symbol = &symbols[27];
                Ok(crypto_symbol.to_string())
            }
        }
    }

    pub async fn execute_trade(&self, trade: TradeRequest) -> Result<TradeResponse> {
            let url = format!("{}/users/current/accounts/{}/trade", env::var("API_URL").context("API_URL is not set in the environment file")?, &self.account_id);

            let response = self.client.post(&url).header("auth-token", &self.api_token).json(&trade).send().await?;
            if !response.status().is_success() {
                let error_text = response.text().await?;
                bail!("Failed to execute trade: {}", error_text);
            }
            let trade_response: TradeResponse = response.json().await?;
            Ok(trade_response)
    }
}
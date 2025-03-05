use reqwest::{Client, header};
use anyhow::{anyhow, bail, Context, Ok, Result};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;


pub struct TradingBot {
    client: Client,
    api_token: String, 
    account_id: String,
    base_url: String,
}

impl TradingBot {
    pub async fn new() -> Result<Self>{
        dotenv().ok();
        let client = Client::new();
        
        // Get credentials from environment
        let api_token = env::var("APIACCESSTOKEN")
            .context("APIACCESSTOKEN not set in environment")?;
        let account_id = env::var("ACCOUNTID")
            .context("ACCOUNTID not set in environment")?;
        
        Ok(Self {
            client,
            api_token,
            account_id,
            base_url: "https://mt-client-api-v1.london.agiliumtrade.ai".to_string(),
        })
    }

    pub async fn check_balance(self,) -> Result<f64>{
        let url = format!("{}/users/current/accounts/{}/account-information", 
        self.base_url, self.account_id);

        // Make the request with the authorization header
        let response = self.client.get(&url)
        .header("auth-token", self.api_token)
        .send()
        .await?;

        if !response.status().is_success() {
        let error_text = response.text().await?;
        bail!("Failed to get account info: {}", error_text);
        }

        // Parse the response to get balance
        let account_info: serde_json::Value = response.json().await?;
        let balance = account_info["balance"].as_f64()
        .unwrap_or(0.0);

        Ok(balance)
    }
}
pub mod account;
pub mod funding;
pub mod models;
pub mod trade;

use chrono::{SecondsFormat, Utc};
use hmac_sha256::HMAC;
use reqwest;
use base64::encode;
use url::Url;

pub struct Client {
    url: String,
    apikey: String,
    secretkey: String,
    passphrase: String,
}

impl Client {
    pub fn new(apikey: &str, secretkey: &str, passphrase: &str) -> anyhow::Result<Self> {
        let s = Self {
            url: "https://www.okex.com".to_string(),
            apikey: apikey.to_string(),
            secretkey: secretkey.to_string(),
            passphrase: passphrase.to_string(),
        };

        Ok(s)
    }

    async fn sign(&mut self, timestamp: &str, method: &str, request_path: &str, body: Option<&String>) -> anyhow::Result<String> {
        match body {
            Some(s) => {
                let sign_msg = format!("{}{}{}{}", timestamp, method, request_path, s);
                let signature = HMAC::mac(sign_msg.as_bytes(), self.secretkey.as_bytes());
                let signature_base64 = encode(signature);

                Ok(signature_base64)
            }
            None => {
                let sign_msg = format!("{}{}{}", timestamp, method, request_path);
                let signature = HMAC::mac(sign_msg.as_bytes(), self.secretkey.as_bytes());
                let signature_base64 = encode(signature);

                Ok(signature_base64)
            }
        }
    }

    pub async fn post_req(&mut self, method: &str, request_path: &str, body: String) -> anyhow::Result<String> {
        let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let sign = self.sign(&timestamp, method, &request_path, Some(&body)).await?;
        let endpoint = Url::parse(&format!("{}{}", self.url, request_path))?;
        let new_post = reqwest::Client::new()
            .post(endpoint)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .header("x-simulated-trading", "1")
            .header("OK-ACCESS-KEY", &self.apikey)
            .header("OK-ACCESS-SIGN", &sign)
            .header("OK-ACCESS-TIMESTAMP", timestamp)
            .header("OK-ACCESS-PASSPHRASE", &self.passphrase)
            .body(body)
            .send()
            .await?
            .text()
            .await?;

        Ok(new_post)
    }

    pub async fn get_req(&mut self, method: &str, request_path: &str) -> anyhow::Result<String> {
        let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let sign = self.sign(&timestamp, method, &request_path, None).await?;
        let endpoint = Url::parse(&format!("{}{}", self.url, request_path))?;
        let new_get = reqwest::Client::new()
            .get(endpoint)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .header("x-simulated-trading", "1")
            .header("OK-ACCESS-KEY", &self.apikey)
            .header("OK-ACCESS-SIGN", &sign)
            .header("OK-ACCESS-TIMESTAMP", timestamp)
            .header("OK-ACCESS-PASSPHRASE", &self.passphrase)
            .send()
            .await?
            .text()
            .await?;

        Ok(new_get)
    }
}

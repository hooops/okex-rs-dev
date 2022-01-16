use std::collections::HashMap;
use crate::rest::Client;
use crate::utils::parse_params_to_str;

pub async fn balances(
    client: &mut Client,
    map: Option<HashMap<&str, &str>>
) -> anyhow::Result<String> {
    let request_path = "/api/v5/asset/balances";
    match map {
        Some(s) => {
            let url = parse_params_to_str(&s)?;
            let s = format!("{}{}", request_path, url);
            let response = client.get_req("GET", s.as_str()).await?;

            Ok(response)
        }
        None => {
            let response = client.get_req("GET", request_path).await?;

            Ok(response)
        }
    }

}

pub async fn asset_valuation(
    client: &mut Client,
    map: Option<HashMap<&str, &str>>
) -> anyhow::Result<String> {
    let request_path = "/api/v5/asset/asset-valuation";
    match map {
        Some(s) => {
            let url = parse_params_to_str(&s)?;
            let s = format!("{}{}", request_path, url);
            let response = client.get_req("GET", s.as_str()).await?;

            Ok(response)
        }
        None => {
            let response = client.get_req("GET", request_path).await?;

            Ok(response)
        }
    }

}

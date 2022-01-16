use std::collections::HashMap;
use crate::rest::Client;
use crate::rest::models::{MarginBalance, SetLeverage, SetPositionMode};
use crate::utils::parse_params_to_str;

pub async fn balance(
    client: &mut Client,
    map: Option<HashMap<&str, &str>>
) -> anyhow::Result<String> {
    let request_path = "/api/v5/account/balance";
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

pub async fn positions(
    client: &mut Client,
    map: Option<HashMap<&str, &str>>
) -> anyhow::Result<String> {
    let request_path = "/api/v5/account/positions";
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

pub async fn set_position_mode(
    client: &mut Client,
    pos_mode: SetPositionMode
) -> anyhow::Result<String> {
    let body = serde_json::to_string(&pos_mode)?;
    let response = client.post_req("POST", "/api/v5/account/set-position-mode", body).await?;

    Ok(response)
}

pub async fn set_leverage(
    client: &mut Client,
    set_leverage: SetLeverage
) -> anyhow::Result<String> {
    let body = serde_json::to_string(&set_leverage)?;
    let response = client.post_req("POST", "/api/v5/account/set-leverage", body).await?;

    Ok(response)
}

pub async fn max_size(
    client: &mut Client,
    map: HashMap<&str, &str>
) -> anyhow::Result<String> {
    let request_path = "/api/v5/account/max-size";
    let url = parse_params_to_str(&map)?;
    let s = format!("{}{}", request_path, url);
    let response = client.get_req("GET", s.as_str()).await?;

    Ok(response)
}

pub async fn max_avail_size(
    client: &mut Client,
    map: HashMap<&str, &str>
) -> anyhow::Result<String> {
    let request_path = "/api/v5/account/max-avail-size";
    let url = parse_params_to_str(&map)?;
    let s = format!("{}{}", request_path, url);
    let response = client.get_req("GET", s.as_str()).await?;

    Ok(response)
}

pub async fn margin_balance(
    client: &mut Client,
    margin_balance: MarginBalance
) -> anyhow::Result<String> {
    let body = serde_json::to_string(&margin_balance)?;
    let response = client.post_req("POST", "/api/v5/account/position/margin-balance", body).await?;

    Ok(response)
}

pub async fn leverage_info(
    client: &mut Client,
    map: HashMap<&str, &str>
) -> anyhow::Result<String> {
    let request_path = "/api/v5/account/leverage-info";
    let url = parse_params_to_str(&map)?;
    let s = format!("{}{}", request_path, url);
    let response = client.get_req("GET", s.as_str()).await?;

    Ok(response)
}

pub async fn max_loan(
    client: &mut Client,
    map: HashMap<&str, &str>
) -> anyhow::Result<String> {
    let request_path = "/api/v5/account/max-loan";
    let url = parse_params_to_str(&map)?;
    let s = format!("{}{}", request_path, url);
    let response = client.get_req("GET", s.as_str()).await?;

    Ok(response)
}

pub async fn trade_fee(
    client: &mut Client,
    map: HashMap<&str, &str>
) -> anyhow::Result<String> {
    let request_path = "/api/v5/account/trade-fee";
    let url = parse_params_to_str(&map)?;
    let s = format!("{}{}", request_path, url);
    let response = client.get_req("GET", s.as_str()).await?;

    Ok(response)
}
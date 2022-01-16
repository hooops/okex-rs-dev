use std::collections::HashMap;
use crate::rest::Client;
use crate::rest::models::{AmendOrder, CancelOrder, ClosePosition, Order};
use crate::utils::parse_params_to_str;

pub async fn order(
    client: &mut Client,
    order: Order
) -> anyhow::Result<String> {
    let body = serde_json::to_string(&order)?;
    let response = client.post_req("POST", "/api/v5/trade/order", body).await?;

    Ok(response)
}

pub async fn batch_orders(
    client: &mut Client,
    list: Vec<Order>
) -> anyhow::Result<String> {
    let body = serde_json::to_string(&list)?;
    let response = client.post_req("POST", "/api/v5/trade/batch-orders", body).await?;

    Ok(response)
}

pub async fn cancel_order(
    client: &mut Client,
    cancel_order: CancelOrder
) -> anyhow::Result<String> {
    let body = serde_json::to_string(&cancel_order)?;
    let response = client.post_req("POST", "/api/v5/trade/cancel-order", body).await?;

    Ok(response)
}

pub async fn cancel_batch_orders(
    client: &mut Client,
    list: Vec<CancelOrder>
) -> anyhow::Result<String> {
    let body = serde_json::to_string(&list)?;
    let response = client.post_req("POST", "/api/v5/trade/cancel-batch-orders", body).await?;

    Ok(response)
}

pub async fn amend_order(
    client: &mut Client,
    amend_order: AmendOrder
) -> anyhow::Result<String> {
    let body = serde_json::to_string(&amend_order)?;
    let response = client.post_req("POST", "/api/v5/trade/amend-order", body).await?;

    Ok(response)
}

pub async fn amend_batch_orders(
    client: &mut Client,
    list: Vec<AmendOrder>
) -> anyhow::Result<String> {
    let body = serde_json::to_string(&list)?;
    let response = client.post_req("POST", "/api/v5/trade/amend-batch-orders", body).await?;

    Ok(response)
}

pub async fn close_position(
    client: &mut Client,
    close_position: ClosePosition
) -> anyhow::Result<String> {
    let body = serde_json::to_string(&close_position)?;
    let response = client.post_req("POST", "/api/v5/trade/close-position", body).await?;

    Ok(response)
}

pub async fn get_orders(
    client: &mut Client,
    map: HashMap<&str, &str>
) -> anyhow::Result<String> {
    let request_path = "/api/v5/trade/order";
    let url = parse_params_to_str(&map)?;
    let s = format!("{}{}", request_path, url);
    let response = client.get_req("GET", s.as_str()).await?;

    Ok(response)
}

pub async fn orders_pending(
    client: &mut Client,
    map: HashMap<&str, &str>
) -> anyhow::Result<String> {
    let request_path = "/api/v5/trade/orders-pending";
    let url = parse_params_to_str(&map)?;
    let s = format!("{}{}", request_path, url);
    let response = client.get_req("GET", s.as_str()).await?;

    Ok(response)
}


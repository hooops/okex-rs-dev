use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "tdMode")]
    pub td_mode: String,
    pub ccy: Option<String>,
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: Option<String>,
    pub tag: Option<String>,
    pub side: String,
    #[serde(rename = "posSide")]
    pub pos_side: Option<String>,
    #[serde(rename = "ordType")]
    pub ord_type: String,
    pub sz: String,
    pub px: Option<String>,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "tgtCcy")]
    pub tgt_ccy: Option<String>,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelOrder {
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "ordId")]
    pub ord_id: Option<String>,
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: Option<String>,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AmendOrder {
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "cxlOnFail")]
    pub cxl_on_fail: Option<bool>,
    #[serde(rename = "ordId")]
    pub ord_id: Option<String>,
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: Option<String>,
    #[serde(rename = "reqId")]
    pub req_id: Option<String>,
    #[serde(rename = "newSz")]
    pub new_sz: Option<String>,
    #[serde(rename = "newPx")]
    pub new_px: Option<String>,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClosePosition {
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "posSide")]
    pub pos_side: Option<String>,
    #[serde(rename = "mgnMode")]
    pub mgn_mode: String,
    #[serde(rename = "ccy")]
    pub ccy: Option<String>,
    #[serde(rename = "autoCxl")]
    pub auto_cxl: Option<bool>,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetPositionMode {
    #[serde(rename = "posMode")]
    pub pos_mode: String,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetLeverage {
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
    #[serde(rename = "ccy")]
    pub ccy: Option<String>,
    #[serde(rename = "lever")]
    pub lever: String,
    #[serde(rename = "mgnMode")]
    pub mgn_mode: String,
    #[serde(rename = "posSide")]
    pub pos_side: Option<String>,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarginBalance {
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "posSide")]
    pub pos_side: String,
    #[serde(rename = "type")]
    pub rename_type: String,
    #[serde(rename = "amt")]
    pub amt: String,
    #[serde(rename = "ccy")]
    pub ccy: Option<String>,
    #[serde(rename = "auto")]
    pub auto: Option<String>,
    #[serde(rename = "loanTrans")]
    pub loan_trans: Option<bool>,

}

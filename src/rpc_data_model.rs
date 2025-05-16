use crate::domain_data_model;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct EstimateGasParams {
    from: Option<String>,
    to: String,
    value: Option<String>,
    data: Option<String>,
}
impl From<&domain_data_model::EstimateGas> for EstimateGasParams {
    fn from(value: &domain_data_model::EstimateGas) -> Self {
        Self {
            from: value.from.clone(),
            to: value.to.clone(),
            value: value.value.clone(),
            data: value.data.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct BlockParam(String);
impl From<&domain_data_model::EstimateGas> for BlockParam {
    fn from(value: &domain_data_model::EstimateGas) -> Self {
        match &value.block {
            Some(v) => BlockParam(v.clone()),
            None => BlockParam(String::from("latest")),
        }
    }
}

#[derive(Serialize)]
pub struct GenericMessage {
    id: u64,
    jsonrpc: String,
    method: String,
    params: Vec<Value>,
}

pub fn prepare_eth_estimate_gas_body(
    data: &domain_data_model::EstimateGas,
) -> serde_json::Result<serde_json::Value> {
    let x = GenericMessage {
        id: 0,
        jsonrpc: String::from("2.0"),
        method: String::from("eth_estimateGas"),
        params: vec![
            serde_json::to_value(EstimateGasParams::from(data))?,
            serde_json::to_value(BlockParam::from(data))?,
        ],
    };
    serde_json::to_value(x)
}

pub struct EstimateGasResult {
    pub value: u64,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct GenericResponse {
    id: u64,
    jsonrpc: String,
    result: Option<Value>,
    error: Option<Value>,
}

pub fn parse_eth_estimate_gas_response(
    data: &str,
) -> std::result::Result<EstimateGasResult, String> {
    let d = serde_json::from_str::<GenericResponse>(data).map_err(|e| e.to_string())?;

    if let Some(err) = d.error {
        Err(err.to_string())
    } else if let Some(res) = d.result {
        if let Some(s) = res.as_str() {
            let s_lower = s.to_ascii_lowercase();
            let without_prefix = s_lower.trim_start_matches("0x");
            let value = u64::from_str_radix(without_prefix, 16).map_err(|e| e.to_string())?;
            Ok(EstimateGasResult { value })
        } else {
            Err(String::from("Expected json string"))
        }
    } else {
        Err(String::from("Expected error or result fields"))
    }
}

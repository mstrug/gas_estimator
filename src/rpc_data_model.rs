/// This file contains definition of the data model of RPC node API calls.

use crate::domain_data_model;
use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Serialize)]
/// Generic RPC call message.
pub struct GenericMessage {
    id: u64,
    jsonrpc: String,
    method: String,
    params: Vec<Value>,
}
impl Default for GenericMessage {
    fn default() -> Self {
        Self { 
            id: 0, 
            jsonrpc: String::from("2.0"), 
            method: String::from("eth_estimateGas"), 
            params: Vec::new()
        }
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
/// Generic RPC call message response.
pub struct GenericResponse {
    id: u64,
    jsonrpc: String,
    result: Option<Value>,
    error: Option<Value>,
}


#[derive(Serialize)]
/// Used for building eth_estimateGas call params.
pub struct EstimateGasParams {
    from: Option<String>,
    to: String,
    value: Option<String>,
    data: Option<String>,
}
/// Converting domain data model into RPC data model.
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
/// Used for building eth_estimateGas call params.
pub struct BlockParam(String);
/// Converting domain data model into RPC data model.
impl From<&domain_data_model::EstimateGas> for BlockParam {
    fn from(value: &domain_data_model::EstimateGas) -> Self {
        match &value.block {
            Some(v) => BlockParam(v.clone()),
            None => BlockParam(String::from("latest")),
        }
    }
}

/// Represents results of eth_estimateGas call response.
pub struct EstimateGasResult {
    pub value: u64,
}


/// Builds eth_estimateGas call message body.
pub fn prepare_eth_estimate_gas_body(
    data: &domain_data_model::EstimateGas,
) -> serde_json::Result<serde_json::Value> {
    let x = GenericMessage {
        params: vec![
            serde_json::to_value(EstimateGasParams::from(data))?,
            serde_json::to_value(BlockParam::from(data))?,
        ],
        ..Default::default()
    };
    serde_json::to_value(x)
}

/// Parses eth_estimateGas call response.
pub fn parse_eth_estimate_gas_response(
    data: &str,
) -> std::result::Result<EstimateGasResult, String> {
    let d = serde_json::from_str::<GenericResponse>(data).map_err(|e| e.to_string())?;

    if let Some(err) = d.error {
        Err(format!("RPC node error: {}", err))
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

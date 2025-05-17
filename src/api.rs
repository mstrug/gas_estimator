/// This file contains definition of the API endpoints data model and function handlers.
use crate::{app::App, domain_data_model};
use axum::{Json, extract::State, http::StatusCode, response::Html};
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
/// Json body message for /estimate endpoint.
/// On success it returns estimated gas value.
pub struct EstimateInputData {
    /// Transaction sender account in form of hexadecimal string.
    /// Field is optional.
    /// Example: 0xabc1234567890abc1234567890abc1234567890f
    #[serde(default, deserialize_with = "non_empty_hex_string")]
    from: Option<String>,

    /// Transaction destination in form of hexadecimal string.
    /// Example: 0xabc1234567890abc1234567890abc1234567890f
    #[serde(deserialize_with = "hex_string")]
    to: String,

    /// Ether to send in the transaction in form of hexadecimal string.
    /// Field is optional.
    /// Examples: 0x1
    #[serde(default, deserialize_with = "non_empty_hex_string")]
    value: Option<String>,

    /// Call data in form of hexadecimal string.
    /// Field is optional.
    /// Example: 0xd0e30db0
    #[serde(default, deserialize_with = "non_empty_hex_string")]
    data: Option<String>,

    /// Block at which estimation should be done. Hexadeciaml value or string: earliest, finalized, safe, latest, or pending.
    /// Field is optional, defaults to 'latest'.
    /// Example: 0x1572338
    #[serde(default, deserialize_with = "validate_block")]
    block: Option<String>,
}

/// Validates hex value
fn validate_hex(v: &str) -> Result<String, &'static str> {
    // Validate format
    if !v.starts_with("0x") && !v.starts_with("0X") {
        // contains prefix
        return Err("Invalid format - missing 0x perfix");
    }
    if v.len() <= 2 {
        // contains only prefix
        return Err("Invalid format - value too short");
    }
    Ok(v.to_owned())
}

/// Used for handling conversion of json strings to valid hex value or None
fn non_empty_hex_string<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    use serde::Deserialize;
    let o: Option<String> = Option::deserialize(d)?;
    let ret = o.filter(|s| !s.is_empty());
    if let Some(v) = ret {
        Ok(Some(validate_hex(&v).map_err(serde::de::Error::custom)?))
    } else {
        Ok(ret)
    }
}

/// Used for handling conversion of json strings to valid hex value
fn hex_string<'de, D: Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    use serde::Deserialize;
    let s: String = String::deserialize(d)?;
    validate_hex(&s).map_err(serde::de::Error::custom)
}

/// Used for handling conversion of json strings to valid block value
fn validate_block<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> { 
    use serde::Deserialize;
    let o: Option<String> = Option::deserialize(d)?;
    let ret = o.filter(|s| !s.is_empty());
    if let Some(v) = ret {
        match v.to_ascii_lowercase().as_str() {
            "earliest" | "finalized" | "safe" | "latest" | "pending" => Ok(Some(v)),
            _ => Ok(Some(validate_hex(&v).map_err(serde::de::Error::custom)?))
        }
    } else {
        Ok(None)
    }
}


#[allow(clippy::from_over_into)]
/// Converting EstimateGas into the domain data model
impl Into<domain_data_model::EstimateGas> for EstimateInputData {
    fn into(self) -> domain_data_model::EstimateGas {
        domain_data_model::EstimateGas {
            from: self.from,
            to: self.to,
            value: self.value,
            data: self.data,
            block: self.block,
        }
    }
}

/// /estimate endpoint handler
pub async fn estimate(
    State(state): State<App>,
    Json(payload): Json<EstimateInputData>,
) -> (StatusCode, String) {
    log::info!("Endpoint: estimate: {:?}", payload);

    match state.estimate_gas(payload.into()).await {
        Ok(val) => (StatusCode::OK, val.to_string()),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()),
    }
}

/// Root endpoint handler - serves simple html website
pub async fn root() -> Html<&'static str> {
    log::info!("Requested root");
    Html::from(include_str!("html/main.html"))
}

/// /version endpoint handler
pub async fn version() -> &'static str {
    log::info!("Requested version");
    App::version()
}

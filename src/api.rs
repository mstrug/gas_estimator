use crate::{App, domain_data_model};
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;

pub async fn root() -> &'static str {
    log::info!("Requested root");
    "Wrong API call"
}

pub async fn version() -> &'static str {
    log::info!("Requested version");
    App::version()
}

#[derive(Deserialize, Debug)]
/// Json body message for /estimate endpoint.
/// On success it returns estimated gas value.
pub struct EstimateInputData {
    /// Transaction sender account in form of hexadecimal string.
    /// Field is optional.
    /// Example: 0xabc1234567890abc1234567890abc1234567890f
    from: Option<String>,

    /// Transaction destination in form of hexadecimal string.
    /// Example: 0xabc1234567890abc1234567890abc1234567890f
    to: String,

    /// Ether to send in the transaction in form of wei or number with unit.
    /// Field is optional.
    /// Examples: 10, 0.1ether, 1gwei
    value: Option<String>,

    /// Call data in form of hexadecimal string.
    /// Field is optional.
    /// Example: 0x095ea7b3000000000000000000000000
    data: Option<String>,

    /// Block at which estimation should be done. Hexadeciaml value or string: earliest, finalized, safe, latest, or pending.
    /// Field is optional, defaults to 'latest'.
    /// Example: 0x1572338
    block: Option<String>,
}

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

pub async fn estimate(
    State(state): State<App>,
    Json(payload): Json<EstimateInputData>,
) -> (StatusCode, String) {
    log::info!("Endpoint: estimate: {:?}", payload);

    match state.estimate_gas(payload.into()).await {
        Ok(val) => (StatusCode::OK, val.to_string()),
        Err(e) => (StatusCode::BAD_REQUEST, e),
    }
}

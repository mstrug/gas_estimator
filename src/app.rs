/// This file contains Applicatino state and logic.
use crate::{config::Config, domain_data_model, rpc_data_model};
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
/// Application errors definition
pub enum AppError {
    #[error("Estimate call failed with an error: `{0}`")]
    EstimateFailed(String),
    #[error("Estimate RPC send failed with an error: `{0}`")]
    EstimateSendFailed(reqwest::Error),
    #[error("Estimate RPC response getting message body failed an with error: `{0}`")]
    EstimateResponseBodyFailed(String),
    #[error("Estimate RPC response call failed with an error: `{0}`")]
    EstimateResponseParseFailed(String),
}

#[derive(Clone)]
/// Application state definition
pub struct App {
    /// HTTP client used for RPC node API calls
    http_client: reqwest::Client,
    /// Configuration used
    config: Config,
}

impl App {
    /// Create new instance of the application
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            config: Config::load(),
        }
    }

    /// Returns server binding address
    pub fn get_bind_address(&self) -> String {
        self.config
            .bind_addr
            .clone()
            .unwrap_or(String::from("0.0.0.0:3000"))
    }

    /// Handles gas estimation calls
    pub async fn estimate_gas(
        &self,
        params: domain_data_model::EstimateGas,
    ) -> Result<u64, AppError> {
        let res = self
            .http_client
            .post(self.config.rpc_url.clone())
            .body(
                rpc_data_model::prepare_eth_estimate_gas_body(&params)
                    .map_err(|e| AppError::EstimateFailed(e.to_string()))?
                    .to_string(),
            )
            .send()
            .await
            .map_err(AppError::EstimateSendFailed)?;

        if res.status().is_success() {
            let body = res
                .text()
                .await
                .map_err(|e| AppError::EstimateResponseBodyFailed(e.to_string()))?;
            log::info!("rpc_call Response body: {}", body);
            let gas = rpc_data_model::parse_eth_estimate_gas_response(&body)
                .map_err(AppError::EstimateResponseParseFailed)?;
            Ok(gas.value)
        } else {
            let error = res.status().to_string();
            log::warn!("rpc_call Response error: {}", error);
            Err(AppError::EstimateFailed(error))
        }
    }

    /// Returns application version
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

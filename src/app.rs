use crate::{config::Config, domain_data_model, rpc_data_model};

#[derive(Clone)]
pub struct App {
    http_client: reqwest::Client,
    config: Config,
}

impl App {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            config: Config::load(),
        }
    }

    pub fn get_bind_address(&self) -> String {
        self.config
            .bind_addr
            .clone()
            .unwrap_or(String::from("0.0.0.0:3000"))
    }

    pub async fn estimate_gas(
        &self,
        params: domain_data_model::EstimateGas,
    ) -> Result<u64, String> {
        let res = self
            .http_client
            .post(self.config.rpc_url.clone())
            .body(
                rpc_data_model::prepare_eth_estimate_gas_body(&params)
                    .map_err(|e| e.to_string())?
                    .to_string(),
            )
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            let body = res.text().await.map_err(|e| e.to_string())?;
            log::info!("rpc_call Response body: {}", body);
            let gas = rpc_data_model::parse_eth_estimate_gas_response(&body)?;
            Ok(gas.value)
        } else {
            let error = res.status().to_string();
            log::warn!("rpc_call Response error: {}", error);
            Err(error)
        }
    }

    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

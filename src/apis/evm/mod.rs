#[allow(dead_code)]
pub mod constants;

use crate::common::api_base::ApiBase;
use crate::avalanche_core::AvalancheCore;
use clru::CLruCache;
use std::num::NonZeroUsize;
use crate::AvalancheError;
use crate::common::json_rpc_api::{JsonRpcApi, JsonRpcResponse, decode_json_rpc_body};

pub struct EvmAPI {
    core: Box<dyn AvalancheCore>,
    cache: CLruCache<String, String>
}

impl ApiBase for EvmAPI {
    fn get_api_base_url(&self) -> &str {
        "/ext/bc/C/avax"
    }
    fn get_cache(&self) -> &CLruCache<String, String> {
        &self.cache
    }
    fn get_core(&self) -> Box<&dyn AvalancheCore> {
        Box::new(&(*self.core))
    }
}

impl JsonRpcApi for EvmAPI {
    fn get_json_rpc_version(&self) -> String {
        String::from("2.0")
    }

    fn get_json_rpc_id(&self) -> u32 {
        1
    }
}

impl EvmAPI {
    pub fn new(core: Box<dyn AvalancheCore>) -> EvmAPI {
        EvmAPI {
            core,
            cache: CLruCache::new(NonZeroUsize::new(2).unwrap())
        }
    }

    pub async fn get_base_fee(&self) -> Result<String, AvalancheError> {
        let response = self.call_method(String::from("eth_baseFee"), None, Some("/ext/bc/C/rpc"), None).await?;
        let body = &hyper::body::to_bytes(response.into_body()).await?;
        let response_formatted: JsonRpcResponse<String> = decode_json_rpc_body("eth_baseFee", body)?;
        Ok(response_formatted.result)
    }

    pub async fn get_max_priority_fee_per_gas(&self) -> Result<String, AvalancheError> {
        let response = self.call_method(String::from("eth_maxPriorityFeePerGas"), None, Some("/ext/bc/C/rpc"), None).await?;
        let body = &hyper::body::to_bytes(response.into_body()).await?;
        let response_formatted: JsonRpcResponse<String> = decode_json_rpc_body("eth_maxPriorityFeePerGas", body)?;
        Ok(response_formatted.result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Avalanche;

    #[tokio::test]
    async fn get_base_fee_works() {
        let avalanche = Avalanche::new(crate::utils::constants::MAINNET_API, 443, Some("https"), None, None, None, None, false).unwrap();
        let evm_api: EvmAPI = EvmAPI::new(Box::new(avalanche));
        assert_eq!(evm_api.get_base_fee().await.unwrap(), "0x5d21dba00");
    }

    #[tokio::test]
    async fn get_max_priority_fee_per_gas_works() {
        let avalanche = Avalanche::new(crate::utils::constants::MAINNET_API, 443, Some("https"), None, None, None, None, false).unwrap();
        let evm_api: EvmAPI = EvmAPI::new(Box::new(avalanche));
        assert_eq!(evm_api.get_max_priority_fee_per_gas().await.unwrap(), "0x0");
    }
}


use crate::common::json_rpc_api::JsonRpcApi;
use crate::common::api_base::ApiBase;
use crate::Avalanche;
use clru::CLruCache;
use std::num::NonZeroUsize;
use std::collections::HashMap;
use crate::errors::AvalancheError;
use serde::{Serialize, Deserialize};
use crate::common::json_rpc_api::JsonRpcResponse;

// TODO: Be able to store any AvalancheCore
pub struct InfoAPI {
    core: Avalanche,
    cache: CLruCache<String, String>
}

impl ApiBase for InfoAPI {
    type Core = Avalanche;
    fn get_api_base_url(&self) -> &str {
        "/ext/info"
    }
    fn get_cache(&self) -> &CLruCache<String, String> {
        &self.cache
    }
    fn get_core(&self) -> &Avalanche {
        &self.core
    }
}

#[derive(Serialize, Deserialize)]
struct ResponseGetBlockchainID {
    #[serde(alias = "blockchainID")]
    pub blockchain_id: String
}

impl JsonRpcApi for InfoAPI {
    fn get_json_rpc_version(&self) -> String {
        "2.0".to_string()
    }

    fn get_json_rpc_id(&self) -> u32 {
        1
    }
}

impl InfoAPI {
    pub fn new(core: Avalanche) -> InfoAPI {
        InfoAPI {
            core: core,
            cache: CLruCache::new(NonZeroUsize::new(2).unwrap())
        }
    }
    pub async fn get_blockchain_id(&self, alias: &str) -> Result<String, AvalancheError> {
        let mut params = HashMap::new();
        params.insert(String::from("alias"), String::from(alias));
        let response = self.call_method(String::from("info.getBlockchainID"), Some(params.clone()), None, None).await?;
        //TODO: Better error management
        let response_formatted: JsonRpcResponse<ResponseGetBlockchainID> = serde_json::from_slice(&hyper::body::to_bytes(response.into_body()).await?).unwrap();
        Ok(response_formatted.result.blockchain_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_blockchain_id_works() {
        let avalanche = Avalanche::new(String::from(crate::utils::constants::MAINNET_API), 443, Some("https"), None, None, None, None, false).unwrap();
        let info_api: InfoAPI = InfoAPI::new(avalanche);
        assert_eq!(info_api.get_blockchain_id("X").await.unwrap(), "2oYMBNV4eNHyqk2fjjV5nVQLDbtmNJzq5s3qs3Lo6ftnC6FByM");
    }
}
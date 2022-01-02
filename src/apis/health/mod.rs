use crate::common::api_base::ApiBase;
use crate::avalanche_core::AvalancheCore;
use clru::CLruCache;
use std::num::NonZeroUsize;
use std::collections::HashMap;
use crate::errors::AvalancheError;
use serde::{Serialize, Deserialize};
use crate::common::json_rpc_api::{JsonRpcApi, JsonRpcResponse, JsonRpcParams, decode_json_rpc_body};

pub struct HealthAPI {
    core: Box<dyn AvalancheCore>,
    cache: CLruCache<String, String>
}

impl ApiBase for HealthAPI {
    fn get_api_base_url(&self) -> &str {
        "/ext/health"
    }
    fn get_cache(&self) -> &CLruCache<String, String> {
        &self.cache
    }
    fn get_core(&self) -> Box<&dyn AvalancheCore> {
        Box::new(&(*self.core))
    }
}

impl JsonRpcApi for HealthAPI {
    fn get_json_rpc_version(&self) -> String {
        String::from("2.0")
    }

    fn get_json_rpc_id(&self) -> u32 {
        1
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Entity {
    message: Option<serde_json::Value>,
    timestamp: String,
    duration: i32,
    #[serde(alias = "contiguousFailures")]
    contiguous_failures: i32,
    #[serde(alias = "timeOfFirstFailure")]
    time_of_first_failure: String
}
  
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Checks {
    #[serde(alias = "C")]
    pub c: Entity,
    #[serde(alias = "P")]
    pub p: Entity,
    #[serde(alias = "X")]
    pub x: Entity,
    #[serde(alias = "isBootstrapped")]
    pub is_bootstrapped: Entity,
    pub network: Entity,
    pub router: Entity
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ResponseHealth {
    pub checks: Checks,
    pub healthy: bool
}

impl HealthAPI {
    pub fn new(core: Box<dyn AvalancheCore>) -> HealthAPI {
        HealthAPI {
            core,
            cache: CLruCache::new(NonZeroUsize::new(2).unwrap())
        }
    }
    pub async fn health(&self, alias: &'static str) -> Result<ResponseHealth, AvalancheError> {
        let mut params = HashMap::new();
        params.insert(String::from("alias"), JsonRpcParams::Str(alias));
        let response = self.call_method("health.health", Some(params), None, None).await?;
        let body = &hyper::body::to_bytes(response.into_body()).await?;
        let response_formatted: JsonRpcResponse<ResponseHealth> = decode_json_rpc_body("health.health", body)?;
        Ok(response_formatted.result)
    }
}
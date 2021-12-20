#[allow(dead_code)]
pub mod constants;

use crate::common::api_base::ApiBase;
use crate::avalanche_core::AvalancheCore;
use clru::CLruCache;
use std::num::NonZeroUsize;
use std::collections::HashMap;
use crate::errors::AvalancheError;
use serde::{Serialize, Deserialize};
use crate::common::json_rpc_api::{JsonRpcApi, JsonRpcResponse, JsonRpcParams, decode_json_rpc_body};

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
}
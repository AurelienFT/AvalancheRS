#[allow(dead_code)]
pub mod constants;

use crate::common::api_base::ApiBase;
use crate::avalanche_core::AvalancheCore;
use clru::CLruCache;
use std::num::NonZeroUsize;
use crate::common::json_rpc_api::{JsonRpcApi};

pub struct EvmAPI<'a> {
    core: Box<dyn AvalancheCore<'a>>,
    cache: CLruCache<String, String>
}

impl<'a> ApiBase<'a> for EvmAPI<'a> {
    fn get_api_base_url(&self) -> &str {
        "/ext/bc/C/avax"
    }
    fn get_cache(&self) -> &CLruCache<String, String> {
        &self.cache
    }
    fn get_core(&self) -> Box<&dyn AvalancheCore<'a>> {
        Box::new(&(*self.core))
    }
}

impl<'a> JsonRpcApi<'a> for EvmAPI<'a> {
    fn get_json_rpc_version(&self) -> String {
        String::from("2.0")
    }

    fn get_json_rpc_id(&self) -> u32 {
        1
    }
}

impl<'a> EvmAPI<'a> {
    pub fn new(core: Box<dyn AvalancheCore>) -> EvmAPI {
        EvmAPI {
            core,
            cache: CLruCache::new(NonZeroUsize::new(2).unwrap())
        }
    }
}
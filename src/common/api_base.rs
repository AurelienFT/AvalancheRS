use clru::CLruCache;
use crate::AvalancheCore;
use core::fmt::Debug;

// TODO: Add namespace to the cache
pub trait ApiBase {
    fn get_api_base_url(&self) -> &str;
    fn get_cache(&self) -> &CLruCache<String, String>;
    fn get_core(&self) -> Box<&dyn AvalancheCore>;
}

impl Debug for dyn ApiBase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "WIP")
    }
}
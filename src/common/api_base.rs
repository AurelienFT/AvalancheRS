use clru::CLruCache;
use crate::AvalancheCore;

// TODO: Add namespace to the cache
pub trait ApiBase {
    type Core: AvalancheCore;
    fn get_api_base_url(&self) -> &str;
    fn get_cache(&self) -> &CLruCache<String, String>;
    fn get_core(&self) -> Self::Core;
}
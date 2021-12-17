use crate::utils::constants::{FALLBACK_HRP, NETWORK_ID_TO_HRP, DEFAULT_NETWORK_ID};

pub fn get_preferred_hrp(network_id: Option<u16>) -> &'static str {
    match network_id {
        Some(network_id) => match NETWORK_ID_TO_HRP.get(&network_id) {
            Some(hrp) => hrp,
            None => NETWORK_ID_TO_HRP.get(&DEFAULT_NETWORK_ID).unwrap(),
        },
        None => FALLBACK_HRP,
    }
}

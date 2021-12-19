use crate::common::json_rpc_api::JsonRpcApi;
use crate::common::api_base::ApiBase;
use crate::Avalanche;
use clru::CLruCache;
use std::num::NonZeroUsize;
use std::collections::HashMap;
use crate::errors::AvalancheError;
use serde::{Serialize, Deserialize};
use crate::common::json_rpc_api::{JsonRpcResponse, JsonRpcParams};
use num_bigint::BigInt;
use std::str::FromStr;

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
struct ResponseJRPCGetBlockchainID {
    #[serde(alias = "blockchainID")]
    pub blockchain_id: String
}

#[derive(Serialize, Deserialize)]
struct ResponseJRPCGetNetworkID {
    #[serde(alias = "networkID")]
    pub network_id: String
}

#[derive(Serialize, Deserialize)]
struct ResponseJRPCGetNetworkName {
    #[serde(alias = "networkName")]
    pub network_name: String
}

#[derive(Serialize, Deserialize)]
struct ResponseJRPCGetNodeID {
    #[serde(alias = "nodeID")]
    pub node_id: String
}

#[derive(Serialize, Deserialize)]
struct ResponseJRPCGetNodeVersion {
    pub version: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseJRPCGetTxFee {
    #[serde(alias = "txFee")]
    pub tx_fee: String,
    #[serde(alias = "creationTxFee")]
    pub creation_tx_fee: String
}

#[derive(Debug, PartialEq)]
pub struct ResponseGetTxFee {
    pub tx_fee: BigInt,
    pub creation_tx_fee: BigInt
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseJRPCIsBootstrapped {
    #[serde(alias = "isBootstrapped")]
    pub is_bootstrapped: bool
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ResponseJRPCPeers {
    #[serde(alias = "numPeers")]
    pub num_peers: String,
    pub peers: Vec<ResponsePeers>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ResponsePeers {
    pub ip: String,
    #[serde(alias = "publicIP")]
    pub public_ip: String,
    #[serde(alias = "nodeID")]
    pub node_id: String,
    pub version: String,
    #[serde(alias = "lastSent")]
    pub last_sent: String,
    #[serde(alias = "lastReceived")]
    pub last_received: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ResponseUptime {
    #[serde(alias = "rewardingStakePercentage")]
    pub rewarding_stake_percentage: String,
    #[serde(alias = "weightedAveragePercentage")]
    pub weighted_average_percentage: String
}

impl JsonRpcApi for InfoAPI {
    fn get_json_rpc_version(&self) -> String {
        String::from("2.0")
    }

    fn get_json_rpc_id(&self) -> u32 {
        1
    }
}

//TODO: Better error management
impl InfoAPI {
    pub fn new(core: Avalanche) -> InfoAPI {
        InfoAPI {
            core: core,
            cache: CLruCache::new(NonZeroUsize::new(2).unwrap())
        }
    }
    pub async fn get_blockchain_id(&self, alias: &str) -> Result<String, AvalancheError> {
        let mut params = HashMap::new();
        params.insert(String::from("alias"), JsonRpcParams::String(String::from(alias)));
        let response = self.call_method(String::from("info.getBlockchainID"), Some(params.clone()), None, None).await?;
        let response_formatted: JsonRpcResponse<ResponseJRPCGetBlockchainID> = serde_json::from_slice(&hyper::body::to_bytes(response.into_body()).await?).unwrap();
        Ok(response_formatted.result.blockchain_id)
    }
    pub async fn get_network_id(&self) -> Result<i32, AvalancheError> {
        let response = self.call_method(String::from("info.getNetworkID"), None, None, None).await?;
        let response_formatted: JsonRpcResponse<ResponseJRPCGetNetworkID> = serde_json::from_slice(&hyper::body::to_bytes(response.into_body()).await?).unwrap();
        Ok(response_formatted.result.network_id.parse::<i32>().unwrap())
    }
    pub async fn get_network_name(&self) -> Result<String, AvalancheError> {
        let response = self.call_method(String::from("info.getNetworkName"), None, None, None).await?;
        let response_formatted: JsonRpcResponse<ResponseJRPCGetNetworkName> = serde_json::from_slice(&hyper::body::to_bytes(response.into_body()).await?).unwrap();
        Ok(response_formatted.result.network_name)
    }
    pub async fn get_node_id(&self) -> Result<String, AvalancheError> {
        let response = self.call_method(String::from("info.getNodeID"), None, None, None).await?;
        let response_formatted: JsonRpcResponse<ResponseJRPCGetNodeID> = serde_json::from_slice(&hyper::body::to_bytes(response.into_body()).await?).unwrap();
        Ok(response_formatted.result.node_id)
    }
    pub async fn get_node_version(&self) -> Result<String, AvalancheError> {
        let response = self.call_method(String::from("info.getNodeVersion"), None, None, None).await?;
        let response_formatted: JsonRpcResponse<ResponseJRPCGetNodeVersion> = serde_json::from_slice(&hyper::body::to_bytes(response.into_body()).await?).unwrap();
        Ok(response_formatted.result.version)
    }
    pub async fn get_tx_fee(&self) -> Result<ResponseGetTxFee, AvalancheError> {
        let response = self.call_method(String::from("info.getTxFee"), None, None, None).await?;
        let response_formatted: JsonRpcResponse<ResponseJRPCGetTxFee> = serde_json::from_slice(&hyper::body::to_bytes(response.into_body()).await?).unwrap();
        Ok(
            ResponseGetTxFee {
                tx_fee: BigInt::from_str(&response_formatted.result.tx_fee).unwrap(),
                creation_tx_fee: BigInt::from_str(&response_formatted.result.creation_tx_fee).unwrap()
            }
        )
    }
    pub async fn is_bootstrapped(&self, chain: &str) -> Result<bool, AvalancheError> {
        let mut params = HashMap::new();
        params.insert(String::from("chain"), JsonRpcParams::String(String::from(chain)));
        let response = self.call_method(String::from("info.isBootstrapped"), Some(params.clone()), None, None).await?;
        let response_formatted: JsonRpcResponse<ResponseJRPCIsBootstrapped> = serde_json::from_slice(&hyper::body::to_bytes(response.into_body()).await?).unwrap();
        Ok(response_formatted.result.is_bootstrapped)
    }
    pub async fn peers(&self, node_ids: Option<Vec<String>>) -> Result<Vec<ResponsePeers>, AvalancheError> {
        let mut params = HashMap::new();
        params.insert(String::from("chain"), JsonRpcParams::VecString(node_ids.unwrap_or(Vec::new())));
        let response = self.call_method(String::from("info.peers"), Some(params.clone()), None, None).await?;
        let response_formatted: JsonRpcResponse<ResponseJRPCPeers> = serde_json::from_slice(&hyper::body::to_bytes(response.into_body()).await?).unwrap();
        Ok(response_formatted.result.peers)
    }
    pub async fn uptime(&self) -> Result<ResponseUptime, AvalancheError> {
        let response = self.call_method(String::from("info.uptime"), None, None, None).await?;
        let response_formatted: JsonRpcResponse<ResponseUptime> = serde_json::from_slice(&hyper::body::to_bytes(response.into_body()).await?).unwrap();
        Ok(response_formatted.result)
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

    #[tokio::test]
    async fn get_network_id_works() {
        let avalanche = Avalanche::new(String::from(crate::utils::constants::MAINNET_API), 443, Some("https"), None, None, None, None, false).unwrap();
        let info_api: InfoAPI = InfoAPI::new(avalanche);
        assert_eq!(info_api.get_network_id().await.unwrap(), 1);
    }

    #[tokio::test]
    async fn get_network_name_works() {
        let avalanche = Avalanche::new(String::from(crate::utils::constants::MAINNET_API), 443, Some("https"), None, None, None, None, false).unwrap();
        let info_api: InfoAPI = InfoAPI::new(avalanche);
        assert_eq!(info_api.get_network_name().await.unwrap(), "mainnet");
    }

    #[tokio::test]
    async fn get_node_version_works() {
        let avalanche = Avalanche::new(String::from(crate::utils::constants::MAINNET_API), 443, Some("https"), None, None, None, None, false).unwrap();
        let info_api: InfoAPI = InfoAPI::new(avalanche);
        assert_eq!(info_api.get_node_version().await.unwrap(), "avalanche/1.7.2");
    }

    #[tokio::test]
    async fn get_tx_works() {
        let avalanche = Avalanche::new(String::from(crate::utils::constants::MAINNET_API), 443, Some("https"), None, None, None, None, false).unwrap();
        let info_api: InfoAPI = InfoAPI::new(avalanche);
        assert_eq!(info_api.get_tx_fee().await.unwrap(), ResponseGetTxFee {
            tx_fee: BigInt::from_str("1000000").unwrap(),
            creation_tx_fee: BigInt::from_str("10000000").unwrap()
        });
    }

    #[tokio::test]
    async fn is_bootstrapped_works() {
        let avalanche = Avalanche::new(String::from(crate::utils::constants::MAINNET_API), 443, Some("https"), None, None, None, None, false).unwrap();
        let info_api: InfoAPI = InfoAPI::new(avalanche);
        assert_eq!(info_api.is_bootstrapped("X").await.unwrap(), true);
    }

    //TODO: Real test but at least it test if it panics
    #[tokio::test]
    async fn peers_works() {
        let avalanche = Avalanche::new(String::from(crate::utils::constants::MAINNET_API), 443, Some("https"), None, None, None, None, false).unwrap();
        let info_api: InfoAPI = InfoAPI::new(avalanche);
        info_api.peers(None).await.unwrap();
    }
}
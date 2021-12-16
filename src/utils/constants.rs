use num_bigint::BigInt;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::ops::{Div, Mul};
use crate::apis::evm::constants::{X, P, C, EnumNetwork, Network};

pub const PRIVATE_KEY_PREFIX: &'static str = "PrivateKey-";
pub const NODE_ID_PREFIX: &'static str = "NodeID-";
pub const PRIMARY_ASSET_ALIAS: &'static str = "AVAX";
pub const MAINNET_API: &'static str = "api.avax.network";
pub const FUJI_API: &'static str = "api.avax-test.network";

lazy_static! {
  pub static ref NETWORK_ID_TO_HRP: HashMap<u16, &'static str> = {
    let mut m = HashMap::new();
    m.insert(0, "custom");
    m.insert(1, "avax");
    m.insert(2, "cascade");
    m.insert(3, "denali");
    m.insert(4, "everest");
    m.insert(5, "fuji");
    m.insert(12345, "local");
    m
  };
  
  pub static ref HRP_TO_NETWORK_ID: HashMap<&'static str, u16> = {
    let mut m = HashMap::new();
    m.insert("custom", 0);
    m.insert("avax", 1);
    m.insert("cascade", 2);
    m.insert("denali", 3);
    m.insert("everest", 4);
    m.insert("fuji", 5);
    m.insert("local", 12345);
    m
  };
  
  pub static ref NETWORK_ID_TO_NETWORK_NAMES: HashMap<u16, Vec<&'static str>> = {
    let mut m = HashMap::new();
    m.insert(0, vec!["Manhattan"]);
    m.insert(1, vec!["Avalanche", "Mainnet"]);
    m.insert(2, vec!["Cascade"]);
    m.insert(3, vec!["Denali"]);
    m.insert(4, vec!["Everest"]);
    m.insert(5, vec!["Fuji", "Testnet"]);
    m.insert(12345, vec!["Local Network"]);
    m
  };
  
  pub static ref NETWORK_NAME_TO_NETWORK_ID: HashMap<&'static str, u16> = {
    let mut m = HashMap::new();
    m.insert("Manhattan", 0);
    m.insert("Avalanche", 1);
    m.insert("Mainnet", 1);
    m.insert("Cascade", 2);
    m.insert("Denali", 3);
    m.insert("Everest", 4);
    m.insert("Fuji", 5);
    m.insert("Testnet", 5);
    m.insert("Local Network", 12345);
    m
  };
}

pub const FALLBACK_HRP: &'static str = "custom";
pub const FALLBACK_NETWORK_NAME: &'static str = "Custom Network";
pub const FALLBACK_EVM_CHAIN_ID: u16 = 43112;

pub const DEFAULT_NETWORK_ID: u16 = 1;

pub const PLATFORM_CHAIN_ID: &'static str = "11111111111111111111111111111111LpoYY";
pub const PRIMARY_NETWORK_ID: &'static str = "11111111111111111111111111111111LpoYY";

pub const X_CHAIN_ALIAS: &'static str = "X";
pub const C_CHAIN_ALIAS: &'static str = "C";
pub const P_CHAIN_ALIAS: &'static str = "P";
pub const X_CHAIN_VM_NAME: &'static str = "avm";
pub const C_CHAIN_VM_NAME: &'static str = "evm";
pub const P_CHAIN_VM_NAME: &'static str = "platformvm";

pub const DEFAULT_LOCAL_GENESIS_PRIVATE_KEY: &'static str = "ewoqjP7PxY4yr3iLTpLisriqt94hdyDFNgchSxGGztUrTXtNN";
pub const DEFAULT_EVM_LOCAL_GENESIS_PRIVATE_KEY: &'static str = "0x56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027";
pub const DEFAULT_EVM_LOCAL_GENESIS_ADDRESS: &'static str = "0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC";
pub const MNEMONIC: &'static str = "output tooth keep tooth bracket fox city sustain blood raise install pond stem reject long scene clap gloom purpose mean music piece unknown light";


lazy_static! {
  pub static ref ONE_AVAX: BigInt = BigInt::parse_bytes("10".as_bytes(), 10).unwrap();
  
  
  pub static ref DECIAVAX: BigInt = {
    ONE_AVAX.clone().div(BigInt::parse_bytes("10".as_bytes(), 10).unwrap())
  };
  
  pub static ref CENTIAVAX: BigInt = {
    ONE_AVAX.clone().div(BigInt::parse_bytes("100".as_bytes(), 10).unwrap())
  };
  
  pub static ref MILLIAVAX: BigInt = {
    ONE_AVAX.clone().div(BigInt::parse_bytes("1000".as_bytes(), 10).unwrap())
  };
  
  pub static ref MICROAVAX: BigInt = {
    ONE_AVAX.clone().div(BigInt::parse_bytes("1000000".as_bytes(), 10).unwrap())
  };
  
  pub static ref NANOAVAX: BigInt = {
    ONE_AVAX.clone().div(BigInt::parse_bytes("1000000000".as_bytes(), 10).unwrap())
  };
  
  pub static ref WEI: BigInt = {
    BigInt::parse_bytes("1".as_bytes(), 10).unwrap()
  };
  
  pub static ref GWEI: BigInt = {
    WEI.clone().mul(BigInt::parse_bytes("1000000000".as_bytes(), 10).unwrap())
  };
  
  pub static ref AVAX_GWEI: BigInt = {
    NANOAVAX.clone()
  };
  
  pub static ref AVAX_STAKE_CAP: BigInt = {
    ONE_AVAX.clone().mul(BigInt::parse_bytes("3000000".as_bytes(), 10).unwrap())
  };

  // Start Manhattan
  pub static ref N_0X: X = X {
    blockchain_id: "2vrXWHgGxh5n3YsLHMV16YVVJTpT4z45Fmb4y3bL6si8kLCyg9",
    alias: X_CHAIN_ALIAS,
    vm: X_CHAIN_VM_NAME,
    fee: Some((*MILLIAVAX).clone()),
    creation_tx_fee: Some((*CENTIAVAX).clone()),
    tx_fee: None,
    avax_asset_id: None
  };
  
  pub static ref N_0P: P = P {
    blockchain_id: PLATFORM_CHAIN_ID,
    alias: P_CHAIN_ALIAS,
    vm: P_CHAIN_VM_NAME,
    fee: Some((*MILLIAVAX).clone()),
    creation_tx_fee: Some((*CENTIAVAX).clone()),
    min_consumption: 0.1,
    max_consumption: 0.12,
    max_staking_duration: BigInt::parse_bytes("31536000".as_bytes(), 10).unwrap(),
    max_supply: BigInt::parse_bytes("720000000".as_bytes(), 10).unwrap().mul((*ONE_AVAX).clone()),
    min_stake: (*ONE_AVAX).clone().mul(BigInt::parse_bytes("2000".as_bytes(), 10).unwrap()),
    min_stake_duration: 2 * 7 * 24 * 60 * 60, // 2 weeks
    max_stake_duration: 365 * 24 * 60 * 60, // 1 year
    min_delegation_stake: (*ONE_AVAX).clone().mul(BigInt::parse_bytes("25".as_bytes(), 10).unwrap()),
    min_delegation_fee: BigInt::parse_bytes("2".as_bytes(), 10).unwrap(),
    avax_asset_id: None,
    tx_fee: None
  };

  pub static ref N_0C: C = C {
    blockchain_id: "2fFZQibQXcd6LTE4rpBPBAkLVXFE91Kit8pgxaBG1mRnh5xqbb",
    alias: C_CHAIN_ALIAS,
    vm: C_CHAIN_VM_NAME,
    fee: Some((*MILLIAVAX).clone()),
    gas_price: (*GWEI).clone().mul(BigInt::parse_bytes("470".as_bytes(), 10).unwrap()), //equivalent to gas price
    chain_id: Some(43111),
    avax_asset_id: None,
    cost_per_signature: None,
    max_gas_price: None,
    min_gas_price: None,
    tx_bytes_gas: None,
    tx_fee: None
  };
  //End Manhattan

}

// Start mainnet
pub const AVAX_ASSET_ID_MAINNET: &'static str = "FvwEAhmxKfeiG8SnEvq42hc6whRyY3EFYAvebMqDNDGCgxN5Z";

lazy_static! {
  pub static ref N_1X: X = X {
    blockchain_id: "2oYMBNV4eNHyqk2fjjV5nVQLDbtmNJzq5s3qs3Lo6ftnC6FByM",
    alias: X_CHAIN_ALIAS,
    vm: X_CHAIN_VM_NAME,
    tx_fee: Some((*MILLIAVAX).clone()),
    creation_tx_fee: Some((*CENTIAVAX).clone()),
    fee: None,
    avax_asset_id: Some(AVAX_ASSET_ID_MAINNET)
  };

  pub static ref N_1P: P = P {
    blockchain_id: PLATFORM_CHAIN_ID,
    alias: P_CHAIN_ALIAS,
    vm: P_CHAIN_VM_NAME,
    tx_fee: Some((*MILLIAVAX).clone()),
    creation_tx_fee: Some((*CENTIAVAX).clone()),
    min_consumption: 0.1,
    max_consumption: 0.12,
    max_staking_duration: BigInt::parse_bytes("31536000".as_bytes(), 10).unwrap(),
    max_supply: BigInt::parse_bytes("720000000".as_bytes(), 10).unwrap().mul((*ONE_AVAX).clone()),
    min_stake: (*ONE_AVAX).clone().mul(BigInt::parse_bytes("2000".as_bytes(), 10).unwrap()),
    min_stake_duration: 2 * 7 * 24 * 60 * 60, // 2 weeks
    max_stake_duration: 365 * 24 * 60 * 60, // 1 year
    min_delegation_stake: (*ONE_AVAX).clone().mul(BigInt::parse_bytes("25".as_bytes(), 10).unwrap()),
    min_delegation_fee: BigInt::parse_bytes("2".as_bytes(), 10).unwrap(),
    avax_asset_id: Some(AVAX_ASSET_ID_MAINNET),
    fee: None
  };

  pub static ref N_1C: C = C {
    blockchain_id: "2q9e4r6Mu3U68nU1fYjgbR6JvwrRx36CohpAX5UQxse55x1Q5",
    alias: C_CHAIN_ALIAS,
    vm: C_CHAIN_VM_NAME,
    tx_bytes_gas: Some(1),
    cost_per_signature: Some(1000),
    // DEPRECATED - txFee
    // WILL BE REMOVED IN NEXT MAJOR VERSION BUMP
    tx_fee: Some((*MILLIAVAX).clone()),
    // DEPRECATED - gasPrice
    // WILL BE REMOVED IN NEXT MAJOR VERSION BUMP
    gas_price: (*GWEI).clone().mul(BigInt::parse_bytes("225".as_bytes(), 10).unwrap()),
    chain_id: Some(43114),
    avax_asset_id: None,
    min_gas_price: Some((*GWEI).clone().mul(BigInt::parse_bytes("25".as_bytes(), 10).unwrap())),
    max_gas_price: Some((*GWEI).clone().mul(BigInt::parse_bytes("1000".as_bytes(), 10).unwrap())),
    fee: None
  };
  //End mainnet

  // Start Cascade
  pub static ref N_2X: X = X {
    blockchain_id: "4ktRjsAKxgMr2aEzv9SWmrU7Xk5FniHUrVCX4P1TZSfTLZWFM",
    alias: X_CHAIN_ALIAS,
    vm: X_CHAIN_VM_NAME,
    tx_fee: Some(BigInt::parse_bytes("0".as_bytes(), 10).unwrap()),
    creation_tx_fee: Some(BigInt::parse_bytes("0".as_bytes(), 10).unwrap()),
    fee: None,
    avax_asset_id: None
  };

  pub static ref N_2P: P = P {
    blockchain_id: PLATFORM_CHAIN_ID,
    alias: P_CHAIN_ALIAS,
    vm: P_CHAIN_VM_NAME,
    tx_fee: Some(BigInt::parse_bytes("0".as_bytes(), 10).unwrap()),
    creation_tx_fee: Some(BigInt::parse_bytes("0".as_bytes(), 10).unwrap()),
    min_consumption: 0.1,
    max_consumption: 0.12,
    max_staking_duration: BigInt::parse_bytes("31536000".as_bytes(), 10).unwrap(),
    max_supply: BigInt::parse_bytes("720000000".as_bytes(), 10).unwrap().mul((*ONE_AVAX).clone()),
    min_stake: (*ONE_AVAX).clone().mul(BigInt::parse_bytes("2000".as_bytes(), 10).unwrap()),
    min_stake_duration: 2 * 7 * 24 * 60 * 60, // 2 weeks
    max_stake_duration: 365 * 24 * 60 * 60, // 1 year
    min_delegation_stake: (*ONE_AVAX).clone().mul(BigInt::parse_bytes("25".as_bytes(), 10).unwrap()),
    min_delegation_fee: BigInt::parse_bytes("2".as_bytes(), 10).unwrap(),
    avax_asset_id: None,
    fee: None
  };

  pub static ref N_2C: C = C {
    blockchain_id: "2mUYSXfLrDtigwbzj1LxKVsHwELghc5sisoXrzJwLqAAQHF4i",
    alias: C_CHAIN_ALIAS,
    vm: C_CHAIN_VM_NAME,
    tx_bytes_gas: None,
    cost_per_signature: None,
    tx_fee: None,
    gas_price: BigInt::parse_bytes("0".as_bytes(), 10).unwrap(),
    chain_id: None,
    avax_asset_id: None,
    min_gas_price: None,
    max_gas_price: None,
    fee: None
  };

  //Start Denali
  pub static ref N_3X: X = X {
    blockchain_id: "rrEWX7gc7D9mwcdrdBxBTdqh1a7WDVsMuadhTZgyXfFcRz45L",
    alias: X_CHAIN_ALIAS,
    vm: X_CHAIN_VM_NAME,
    tx_fee: Some(BigInt::parse_bytes("0".as_bytes(), 10).unwrap()),
    creation_tx_fee: Some(BigInt::parse_bytes("0".as_bytes(), 10).unwrap()),
    fee: None,
    avax_asset_id: None
  };

  pub static ref N_3P: P = P {
    blockchain_id: PLATFORM_CHAIN_ID,
    alias: P_CHAIN_ALIAS,
    vm: P_CHAIN_VM_NAME,
    tx_fee: Some(BigInt::parse_bytes("0".as_bytes(), 10).unwrap()),
    creation_tx_fee: Some(BigInt::parse_bytes("0".as_bytes(), 10).unwrap()),
    min_consumption: 0.1,
    max_consumption: 0.12,
    max_staking_duration: BigInt::parse_bytes("31536000".as_bytes(), 10).unwrap(),
    max_supply: BigInt::parse_bytes("720000000".as_bytes(), 10).unwrap().mul((*ONE_AVAX).clone()),
    min_stake: (*ONE_AVAX).clone().mul(BigInt::parse_bytes("2000".as_bytes(), 10).unwrap()),
    min_stake_duration: 2 * 7 * 24 * 60 * 60, // 2 weeks
    max_stake_duration: 365 * 24 * 60 * 60, // 1 year
    min_delegation_stake: (*ONE_AVAX).clone().mul(BigInt::parse_bytes("25".as_bytes(), 10).unwrap()),
    min_delegation_fee: BigInt::parse_bytes("2".as_bytes(), 10).unwrap(),
    avax_asset_id: None,
    fee: None
  };

  pub static ref N_3C: C = C {
    blockchain_id: "zJytnh96Pc8rM337bBrtMvJDbEdDNjcXG3WkTNCiLp18ergm9",
    alias: C_CHAIN_ALIAS,
    vm: C_CHAIN_VM_NAME,
    tx_bytes_gas: None,
    cost_per_signature: None,
    tx_fee: None,
    gas_price: BigInt::parse_bytes("0".as_bytes(), 10).unwrap(),
    chain_id: None,
    avax_asset_id: None,
    min_gas_price: None,
    max_gas_price: None,
    fee: None
  };
  // End Denali

  // Start Everest
  pub static ref N_4X: X = X {
    blockchain_id: "jnUjZSRt16TcRnZzmh5aMhavwVHz3zBrSN8GfFMTQkzUnoBxC",
    alias: X_CHAIN_ALIAS,
    vm: X_CHAIN_VM_NAME,
    tx_fee: Some((*MILLIAVAX).clone()),
    creation_tx_fee: Some((*CENTIAVAX).clone()),
    fee: None,
    avax_asset_id: None
  };

  pub static ref N_4P: P = P {
    blockchain_id: PLATFORM_CHAIN_ID,
    alias: P_CHAIN_ALIAS,
    vm: P_CHAIN_VM_NAME,
    tx_fee: Some((*MILLIAVAX).clone()),
    creation_tx_fee: Some((*CENTIAVAX).clone()),
    min_consumption: 0.1,
    max_consumption: 0.12,
    max_staking_duration: BigInt::parse_bytes("31536000".as_bytes(), 10).unwrap(),
    max_supply: BigInt::parse_bytes("720000000".as_bytes(), 10).unwrap().mul((*ONE_AVAX).clone()),
    min_stake: (*ONE_AVAX).clone().mul(BigInt::parse_bytes("2000".as_bytes(), 10).unwrap()),
    min_stake_duration: 2 * 7 * 24 * 60 * 60, // 2 weeks
    max_stake_duration: 365 * 24 * 60 * 60, // 1 year
    min_delegation_stake: (*ONE_AVAX).clone().mul(BigInt::parse_bytes("25".as_bytes(), 10).unwrap()),
    min_delegation_fee: BigInt::parse_bytes("2".as_bytes(), 10).unwrap(),
    avax_asset_id: None,
    fee: None
  };

  pub static ref N_4C: C = C {
    blockchain_id: "saMG5YgNsFxzjz4NMkEkt3bAH6hVxWdZkWcEnGB3Z15pcAmsK",
    alias: C_CHAIN_ALIAS,
    vm: C_CHAIN_VM_NAME,
    tx_bytes_gas: None,
    cost_per_signature: None,
    tx_fee: None,
    gas_price: (*GWEI).clone().mul(BigInt::parse_bytes("470".as_bytes(), 10).unwrap()),
    chain_id: Some(43110),
    avax_asset_id: None,
    min_gas_price: None,
    max_gas_price: None,
    fee: None
  };
  // End Everest
}

const AVAX_ASSET_ID_FUJI: &'static str = "U8iRqJoiJm8xZHAacmvYyZVwqQx6uDNtQeP3CQ6fcgQk3JqnK";

lazy_static! {
  // Start Fuji
  pub static ref N_5X: X = X {
    blockchain_id: "2JVSBoinj9C2J33VntvzYtVJNZdN2NKiwwKjcumHUWEb5DbBrm",
    alias: X_CHAIN_ALIAS,
    vm: X_CHAIN_VM_NAME,
    tx_fee: Some((*MILLIAVAX).clone()),
    creation_tx_fee: Some((*CENTIAVAX).clone()),
    fee: None,
    avax_asset_id: Some(AVAX_ASSET_ID_FUJI)
  };

  pub static ref N_5P: P = P {
    blockchain_id: PLATFORM_CHAIN_ID,
    alias: P_CHAIN_ALIAS,
    vm: P_CHAIN_VM_NAME,
    tx_fee: Some((*MILLIAVAX).clone()),
    creation_tx_fee: Some((*CENTIAVAX).clone()),
    min_consumption: 0.1,
    max_consumption: 0.12,
    max_staking_duration: BigInt::parse_bytes("31536000".as_bytes(), 10).unwrap(),
    max_supply: BigInt::parse_bytes("720000000".as_bytes(), 10).unwrap().mul((*ONE_AVAX).clone()),
    min_stake: (*ONE_AVAX).clone(),
    min_stake_duration: 24 * 60 * 60, // 1 day
    max_stake_duration: 365 * 24 * 60 * 60, // 1 year
    min_delegation_stake: (*ONE_AVAX).clone(),
    min_delegation_fee: BigInt::parse_bytes("2".as_bytes(), 10).unwrap(),
    avax_asset_id: Some(AVAX_ASSET_ID_FUJI),
    fee: None
  };

  pub static ref N_5C: C = C {
    blockchain_id: "yH8D7ThNJkxmtkuv2jgBa4P1Rn3Qpr4pPr7QYNfcdoS6k6HWp",
    alias: C_CHAIN_ALIAS,
    vm: C_CHAIN_VM_NAME,
    tx_bytes_gas: Some(1),
    cost_per_signature: Some(1000),
    // DEPRECATED - txFee
    // WILL BE REMOVED IN NEXT MAJOR VERSION BUMP
    tx_fee: Some((*MILLIAVAX).clone()),
    // DEPRECATED - gasPrice
    // WILL BE REMOVED IN NEXT MAJOR VERSION BUMP
    gas_price: (*GWEI).clone().mul(BigInt::parse_bytes("225".as_bytes(), 10).unwrap()),
    chain_id: Some(43113),
    avax_asset_id: None,
    min_gas_price: Some((*GWEI).clone().mul(BigInt::parse_bytes("25".as_bytes(), 10).unwrap())),
    max_gas_price: Some((*GWEI).clone().mul(BigInt::parse_bytes("100".as_bytes(), 10).unwrap())),
    fee: None
  };
  // End Fuji
}

const AVAX_ASSET_ID_LOCAL_NETWORK: &'static str = "2fombhL7aGPwj3KH4bfrmJwW6PVnMobf9Y2fn9GwxiAAJyFDbe";

lazy_static! {
  //Start Local Network
  pub static ref N_12345X: X = {
    let mut n = (*N_5X).clone();
    n.blockchain_id = "2eNy1mUFdmaxXNj1eQHUe7Np4gju9sJsEtWQ4MX3ToiNKuADed";
    n.avax_asset_id = Some(AVAX_ASSET_ID_LOCAL_NETWORK);
    n
  };

  pub static ref N_12345P: P = {
    let mut n = (*N_5P).clone();
    n.blockchain_id = PLATFORM_CHAIN_ID;
    n
  };

  pub static ref N_12345C: C = {
    let mut n = (*N_5C).clone();
    n.blockchain_id = "2CA6j5zYzasynPsFeNoqWkmTCt3VScMvXUZHbfDJ8k3oGzAPtU";
    n.avax_asset_id = Some(AVAX_ASSET_ID_LOCAL_NETWORK);
    n.chain_id = Some(43112);
    n
  };
  //End Local Network
}

#[non_exhaustive]
struct MergeRule;

impl MergeRule {
  pub const INTERSECTION: &'static str = "intersection";
  pub const DIFFERENCE_SELF: &'static str = "differenceSelf";
  pub const DIFFERENCE_NEW: &'static str = "differenceNew";
  pub const SYM_DIFFERENCE: &'static str = "symDifference";
  pub const UNION: &'static str = "union";
  pub const UNION_MINUS_NEW: &'static str = "unionMinusNew";
  pub const UNION_MINUS_SELF: &'static str = "unionMinusSelf";
  pub const ERROR: &'static str = "ERROR";
}

lazy_static! {
  pub static ref NETWORK: HashMap<i32, Network> = {
    let mut m = HashMap::new();

    let mut addresses_0: HashMap<&'static str, EnumNetwork> = HashMap::new();
    addresses_0.insert("2vrXWHgGxh5n3YsLHMV16YVVJTpT4z45Fmb4y3bL6si8kLCyg9", EnumNetwork::X((*N_0X).clone()));
    addresses_0.insert("11111111111111111111111111111111LpoYY", EnumNetwork::P((*N_0P).clone()));
    addresses_0.insert("2fFZQibQXcd6LTE4rpBPBAkLVXFE91Kit8pgxaBG1mRnh5xqbb", EnumNetwork::C((*N_0C).clone()));
    m.insert(0, Network {
      hrp: Some(*NETWORK_ID_TO_HRP.get(&0).unwrap()),
      x: (*N_0X).clone(),
      p: (*N_0P).clone(),
      c: (*N_0C).clone(),
      addresses: addresses_0,
    });

    let mut addresses_1: HashMap<&'static str, EnumNetwork> = HashMap::new();
    addresses_1.insert("2oYMBNV4eNHyqk2fjjV5nVQLDbtmNJzq5s3qs3Lo6ftnC6FByM", EnumNetwork::X((*N_1X).clone()));
    addresses_1.insert("11111111111111111111111111111111LpoYY", EnumNetwork::P((*N_1P).clone()));
    addresses_1.insert("2q9e4r6Mu3U68nU1fYjgbR6JvwrRx36CohpAX5UQxse55x1Q5", EnumNetwork::C((*N_1C).clone()));
    m.insert(1, Network {
      hrp: Some(*NETWORK_ID_TO_HRP.get(&1).unwrap()),
      x: (*N_1X).clone(),
      p: (*N_1P).clone(),
      c: (*N_1C).clone(),
      addresses: addresses_1,
    });

    let mut addresses_2: HashMap<&'static str, EnumNetwork> = HashMap::new();
    addresses_2.insert("4ktRjsAKxgMr2aEzv9SWmrU7Xk5FniHUrVCX4P1TZSfTLZWFM", EnumNetwork::X((*N_2X).clone()));
    addresses_2.insert("11111111111111111111111111111111LpoYY", EnumNetwork::P((*N_2P).clone()));
    addresses_2.insert("2mUYSXfLrDtigwbzj1LxKVsHwELghc5sisoXrzJwLqAAQHF4i", EnumNetwork::C((*N_2C).clone()));
    m.insert(2, Network {
      hrp: Some(*NETWORK_ID_TO_HRP.get(&2).unwrap()),
      x: (*N_2X).clone(),
      p: (*N_2P).clone(),
      c: (*N_2C).clone(),
      addresses: addresses_2,
    });

    let mut addresses_3: HashMap<&'static str, EnumNetwork> = HashMap::new();
    addresses_3.insert("rrEWX7gc7D9mwcdrdBxBTdqh1a7WDVsMuadhTZgyXfFcRz45L", EnumNetwork::X((*N_3X).clone()));
    addresses_3.insert("11111111111111111111111111111111LpoYY", EnumNetwork::P((*N_3P).clone()));
    addresses_3.insert("zJytnh96Pc8rM337bBrtMvJDbEdDNjcXG3WkTNCiLp18ergm9", EnumNetwork::C((*N_3C).clone()));
    m.insert(3, Network {
      hrp: Some(*NETWORK_ID_TO_HRP.get(&3).unwrap()),
      x: (*N_3X).clone(),
      p: (*N_3P).clone(),
      c: (*N_3C).clone(),
      addresses: addresses_3,
    });

    let mut addresses_4: HashMap<&'static str, EnumNetwork> = HashMap::new();
    addresses_4.insert("jnUjZSRt16TcRnZzmh5aMhavwVHz3zBrSN8GfFMTQkzUnoBxC", EnumNetwork::X((*N_4X).clone()));
    addresses_4.insert("11111111111111111111111111111111LpoYY", EnumNetwork::P((*N_4P).clone()));
    addresses_4.insert("saMG5YgNsFxzjz4NMkEkt3bAH6hVxWdZkWcEnGB3Z15pcAmsK", EnumNetwork::C((*N_4C).clone()));
    m.insert(4, Network {
      hrp: Some(*NETWORK_ID_TO_HRP.get(&4).unwrap()),
      x: (*N_4X).clone(),
      p: (*N_4P).clone(),
      c: (*N_4C).clone(),
      addresses: addresses_4,
    });

    let mut addresses_5: HashMap<&'static str, EnumNetwork> = HashMap::new();
    addresses_5.insert("2JVSBoinj9C2J33VntvzYtVJNZdN2NKiwwKjcumHUWEb5DbBrm", EnumNetwork::X((*N_5X).clone()));
    addresses_5.insert("11111111111111111111111111111111LpoYY", EnumNetwork::P((*N_5P).clone()));
    addresses_5.insert("yH8D7ThNJkxmtkuv2jgBa4P1Rn3Qpr4pPr7QYNfcdoS6k6HWp", EnumNetwork::C((*N_5C).clone()));
    m.insert(5, Network {
      hrp: Some(*NETWORK_ID_TO_HRP.get(&5).unwrap()),
      x: (*N_5X).clone(),
      p: (*N_5P).clone(),
      c: (*N_5C).clone(),
      addresses: addresses_5,
    });

    let mut addresses_12345: HashMap<&'static str, EnumNetwork> = HashMap::new();
    addresses_12345.insert("2eNy1mUFdmaxXNj1eQHUe7Np4gju9sJsEtWQ4MX3ToiNKuADed", EnumNetwork::X((*N_12345X).clone()));
    addresses_12345.insert("11111111111111111111111111111111LpoYY", EnumNetwork::P((*N_12345P).clone()));
    addresses_12345.insert("2eNy1mUFdmaxXNj1eQHUe7Np4gju9sJsEtWQ4MX3ToiNKuADed", EnumNetwork::C((*N_12345C).clone()));
    m.insert(12345, Network {
      hrp: Some(*NETWORK_ID_TO_HRP.get(&12345).unwrap()),
      x: (*N_12345X).clone(),
      p: (*N_12345P).clone(),
      c: (*N_12345C).clone(),
      addresses: addresses_12345,
    });
    m
  };
}
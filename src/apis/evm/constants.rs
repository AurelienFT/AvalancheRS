use num_bigint::BigInt;
use std::collections::HashMap;

pub const SEC_P_CREDENTIAL: i8 = 9;
pub const IMPORT_TX: i8 = 0;
pub const EXPORT_TX: i8 = 1;
pub const SEC_P_IN_PUT_ID: i8 = 5;
pub const ASSET_ID_LEN: i8 = 32;
pub const SEC_P_X_FER_OUT_PUT_ID: i8 = 7;
pub const LATEST_CODEC: i8 = 0;
pub const ADDRESS_LENGTH: i8 = 20;

#[derive(Clone)]
pub struct C {
    pub blockchain_id: &'static str,
    pub alias: &'static str,
    pub vm: &'static str,
    pub fee: Option<BigInt>,
    pub gas_price: BigInt,
    pub chain_id: Option<i32>,
    pub min_gas_price: Option<BigInt>,
    pub max_gas_price: Option<BigInt>,
    pub tx_bytes_gas: Option<i32>,
    pub cost_per_signature: Option<i32>,
    pub tx_fee: Option<BigInt>,
    pub avax_asset_id: Option<&'static str>
}

#[derive(Clone)]
pub struct X {
    pub blockchain_id: &'static str,
    pub alias: &'static str,
    pub vm: &'static str,
    pub creation_tx_fee: Option<BigInt>,
    pub avax_asset_id: Option<&'static str>,
    pub tx_fee: Option<BigInt>,
    pub fee: Option<BigInt>
}

#[derive(Clone)]
pub struct P {
    pub blockchain_id: &'static str,
    pub alias: &'static str,
    pub vm: &'static str,
    pub creation_tx_fee: Option<BigInt>,
    pub min_consumption: f32,
    pub max_consumption: f32,
    pub max_staking_duration: BigInt,
    pub max_supply: BigInt,
    pub min_stake: BigInt,
    pub min_stake_duration: i32,
    pub max_stake_duration: i32,
    pub min_delegation_stake: BigInt,
    pub min_delegation_fee: BigInt,
    pub avax_asset_id: Option<&'static str>,
    pub tx_fee: Option<BigInt>,
    pub fee: Option<BigInt>
}

#[derive(Clone)]
pub enum EnumNetwork {
    X(X),
    C(C),
    P(P),
    String(String)
}

#[derive(Clone)]
pub struct Network {
    pub c: C,
    pub hrp: Option<&'static str>,
    pub x: X,
    pub p: P,
    pub addresses: HashMap<&'static str, EnumNetwork>
}

pub type Networks = HashMap<i16, Network>;


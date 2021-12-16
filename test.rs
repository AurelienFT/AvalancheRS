// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct Test {
    #[serde(rename = "$schema")]
    schema: String,

    #[serde(rename = "definitions")]
    definitions: Definitions,

    #[serde(rename = "properties")]
    properties: Properties,

    #[serde(rename = "type")]
    test_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Definitions {
    #[serde(rename = "import(\"/Users/aurelienfoucault/Avalanche/avalanchejs/node_modules/@types/bn.js/index\")")]
    import_users_aurelienfoucault_avalanche_avalanchejs_node_modules_types_bn_js_index: ImportUsersAurelienfoucaultAvalancheAvalanchejsNodeModulesTypesBnJsIndex,
}

#[derive(Serialize, Deserialize)]
pub struct ImportUsersAurelienfoucaultAvalancheAvalanchejsNodeModulesTypesBnJsIndex {
    #[serde(rename = "type")]
    import_users_aurelienfoucault_avalanche_avalanchejs_node_modules_types_bn_js_index_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Properties {
    #[serde(rename = "alias")]
    alias: ImportUsersAurelienfoucaultAvalancheAvalanchejsNodeModulesTypesBnJsIndex,

    #[serde(rename = "avaxAssetID")]
    avax_asset_id: ImportUsersAurelienfoucaultAvalancheAvalanchejsNodeModulesTypesBnJsIndex,

    #[serde(rename = "blockchainID")]
    blockchain_id: ImportUsersAurelienfoucaultAvalancheAvalanchejsNodeModulesTypesBnJsIndex,

    #[serde(rename = "creationTxFee")]
    creation_tx_fee: TxFee,

    #[serde(rename = "fee")]
    fee: Fee,

    #[serde(rename = "txFee")]
    tx_fee: TxFee,

    #[serde(rename = "vm")]
    vm: ImportUsersAurelienfoucaultAvalancheAvalanchejsNodeModulesTypesBnJsIndex,
}

#[derive(Serialize, Deserialize)]
pub struct TxFee {
    #[serde(rename = "anyOf")]
    any_of: Vec<AnyOf>,
}

#[derive(Serialize, Deserialize)]
pub struct AnyOf {
    #[serde(rename = "$ref")]
    any_of_ref: Option<String>,

    #[serde(rename = "type")]
    any_of_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Fee {
    #[serde(rename = "$ref")]
    fee_ref: String,
}

use crate::common::api_base::ApiBase;
use crate::AvalancheError;
use std::collections::HashMap;
use hyper::client::{ResponseFuture};
use serde::{Serialize, Serializer, Deserialize};

#[derive(Deserialize, Clone)]
pub enum JsonRpcParams {
    String(String),
    HashMap(HashMap<String, JsonRpcParams>),
    VecString(Vec<String>)
}

impl Serialize for JsonRpcParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            JsonRpcParams::String(s) => s.serialize(serializer),
            JsonRpcParams::HashMap(h) => h.serialize(serializer),
            JsonRpcParams::VecString(vs) => vs.serialize(serializer),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsonRpcError {
    pub jsonrpc: String,
    pub id: String,
    pub error: JsonRpcSubError
}

#[derive(Serialize, Deserialize)]
pub struct JsonRpcSubError {
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub id: String,
    pub result: T
}

pub trait JsonRpcApi: ApiBase {
    fn get_json_rpc_version(&self) -> String;
    fn get_json_rpc_id(&self) -> u32;
    fn call_method(&self, method: String, params: Option<HashMap<String, JsonRpcParams>>, base_api_url: Option<&str>, headers: Option<HashMap<&str, &str>>) -> ResponseFuture {
        let ep = base_api_url.unwrap_or_else(|| self.get_api_base_url());
        let mut params_call: HashMap<&str, JsonRpcParams> = HashMap::new();
        params_call.insert("id", JsonRpcParams::String(self.get_json_rpc_id().to_string()));
        params_call.insert("method", JsonRpcParams::String(method));
        if let Some(p) = params {
            params_call.insert("params", JsonRpcParams::HashMap(p));
        }
        if self.get_json_rpc_version() != "1.0" {
            params_call.insert("jsonrpc", JsonRpcParams::String(self.get_json_rpc_version()));
        }
        let mut headers_call = match headers {
            Some(h) => h.clone(),
            None => HashMap::new()
        };
        headers_call.insert("Content-Type", "application/json;charset=UTF-8");
        let base_url = format!("{}://{}:{}{}", self.get_core().get_protocol(), self.get_core().get_host(), self.get_core().get_port(), ep);
        let body_string = serde_json::to_vec(&params_call).unwrap();
        self.get_core().post(&base_url, body_string.into(), headers_call)
        //TODO: Add error handling
    }
}

pub fn decode_json_rpc_body<T>(call_name: &str, body: &hyper::body::Bytes) -> Result<JsonRpcResponse<T>, AvalancheError>
where 
    T: serde::de::DeserializeOwned
    {
    match serde_json::from_slice::<JsonRpcResponse<T>>(body) {
        Ok(response) => Ok(response),
        Err(_) => {
            let response = serde_json::from_slice::<JsonRpcError>(body).unwrap();
            Err(AvalancheError::ErrorJsonRpcCall{
                call: String::from(call_name),
                code: response.error.code.to_string(),
                message: response.error.message
            })
        }
    }
}
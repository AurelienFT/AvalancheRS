pub mod apis;
pub mod avalanche_core;
pub mod errors;
pub mod utils;
pub mod common;

use crate::avalanche_core::AvalancheCore;
use crate::errors::AvalancheError;
use crate::utils::constants::{DEFAULT_NETWORK_ID, NETWORK};
use crate::utils::helper_functions::get_preferred_hrp;
use crate::common::api_base::ApiBase;
use crate::apis::info::InfoAPI;
use hyper::client::ResponseFuture;
use hyper::header::{HeaderName, HeaderValue};
use hyper::{Body, Client, Method, Request};
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use url::Url;
use hyper_tls::HttpsConnector;

#[derive(Debug, Default)]
pub struct Avalanche {
    network_id: u16,
    hrp: &'static str,
    protocol: &'static str,
    host: &'static str,
    ip: &'static str,
    port: u32,
    url: String,
    headers: HashMap<String, String>,
    auth: Option<String>,
    apis: HashMap<&'static str, Box<dyn ApiBase>>
}

impl Clone for Avalanche {
    fn clone(&self) -> Self {
        Avalanche {
            network_id: self.network_id,
            hrp: self.hrp,
            protocol: self.protocol,
            host: self.host,
            ip: self.ip,
            port: self.port,
            url: self.url.clone(),
            headers: self.headers.clone(),
            auth: self.auth.clone(),
            apis: HashMap::new()
        }
    }
}

#[allow(clippy::too_many_arguments)]
impl Avalanche {
    // TODO: Maybe change to a builder ?
    pub fn new(
        host: &'static str,
        port: u32,
        protocol: Option<&'static str>,
        network_id: Option<u16>,
        x_chain_id: Option<&'static str>,
        c_chain_id: Option<&'static str>,
        hrp: Option<&'static str>,
        skip_init: bool,
    ) -> Result<Avalanche, AvalancheError> {
        let mut avalanche: Avalanche = Avalanche::default();
        avalanche.set_address(host, port, protocol)?;
        let network_id_resolved = network_id.unwrap_or(DEFAULT_NETWORK_ID);
        let _x_chain_final: &'static str = match x_chain_id {
            Some(x_chain_id_resolved) => {
                x_chain_id_resolved
            }
            None => {
                if NETWORK.contains_key(&network_id_resolved) {
                    NETWORK.get(&network_id_resolved).unwrap().x.blockchain_id
                } else {
                    NETWORK.get(&12345).unwrap().x.blockchain_id
                }
            }
        };
        let _c_chain_final: &'static str = match c_chain_id {
            Some(c_chain_id_resolved) => {
                c_chain_id_resolved
            }
            None => {
                if NETWORK.contains_key(&network_id_resolved) {
                    NETWORK.get(&network_id_resolved).unwrap().c.blockchain_id
                } else {
                    NETWORK.get(&12345).unwrap().c.blockchain_id
                }
            }
        };
        match network_id {
            Some(network_id_resolved) => {
                avalanche.network_id = network_id_resolved;
            }
            None => avalanche.network_id = DEFAULT_NETWORK_ID,
        };
        match hrp {
            Some(hrp_resolved) => {
                avalanche.hrp = hrp_resolved;
            }
            None => {
                avalanche.hrp = get_preferred_hrp(Some(avalanche.network_id));
            }
        };
        if !skip_init {
            avalanche.add_api("info", Box::new(InfoAPI::new(Box::new(avalanche.clone()))));
        }
        Ok(avalanche)
    }
    fn set_header(
        &self,
        mut request: hyper::Request<hyper::Body>,
        headers: HashMap<&str, &str>,
    ) -> hyper::Request<hyper::Body> {
        for (key, value) in headers {
            request.headers_mut().insert(
                HeaderName::from_str(key).unwrap(),
                HeaderValue::from_str(value).unwrap(),
            );
        }
        for (key, value) in self.headers.iter() {
            request.headers_mut().insert(
                HeaderName::from_str(key).unwrap(),
                HeaderValue::from_str(value).unwrap(),
            );
        }
        if let Some(ref token) = self.auth {
            request.headers_mut().insert(
                HeaderName::from_str("Authorization").unwrap(),
                HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
            );
        }
        request
    }
    fn request(
        &self,
        url: &str,
        method: Method,
        get_data: HashMap<&str, &str>,
        post_data: Body,
        headers: HashMap<&str, &str>,
    ) -> ResponseFuture {
        let mut uri = Url::parse(url).unwrap();
        for (key, value) in get_data.iter() {
            uri.query_pairs_mut().append_pair(key, value);
        }
        let mut request = Request::builder()
            .method(method)
            .uri(uri.as_str())
            .body(post_data)
            .expect("request builder");
        request = self.set_header(request, headers);
        let https = HttpsConnector::new();
        if self.get_protocol() == "http" {
            Client::new().request(request)
        } else {
            Client::builder().build::<_, hyper::Body>(https).request(request)
        }
    }
    pub fn info(&self) -> Result<&Box<dyn ApiBase>, AvalancheError> {
        self.apis.get("info").ok_or(AvalancheError::ApiNotInitialized {
            api: String::from("info")
        })
    }
}

impl AvalancheCore for Avalanche {
    fn set_address(
        &mut self,
        host: &'static str,
        port: u32,
        protocol: Option<&'static str>,
    ) -> Result<(), AvalancheError> {
        let re = Regex::new(r"[&#,@+()$~%':*?<>{}]").unwrap(); //TODO: Add "
        let host_resolved = &re.replace_all(host, "");
        let protocol_defined: &'static str = protocol.unwrap_or("http");
        let protocols: Vec<&str> = vec!["http", "https"];
        if !protocols.contains(&protocol_defined) {
            return Err(AvalancheError::BadProtocol);
        }
        self.host = host;
        self.port = port;
        self.protocol = protocol_defined;
        self.url = format!("{}://{}:{}", &protocol_defined, &host_resolved, &port);
        Ok(())
    }
    fn get_protocol(&self) -> &str {
        self.protocol
    }
    fn get_host(&self) -> &str {
        self.host
    }
    fn get_ip(&self) -> &str {
        self.ip
    }
    fn get_port(&self) -> u32 {
        self.port
    }
    fn get_url(&self) -> &str {
        &self.url
    }
    fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
    fn get_network_id(&self) -> u16 {
        self.network_id
    }
    fn set_network_id(&mut self, network_id: u16) {
        self.network_id = network_id;
        self.hrp = get_preferred_hrp(Some(network_id));
    }
    fn get_hrp(&self) -> &str {
        self.hrp
    }
    fn set_hrp(&mut self, hrp: &'static str) {
        self.hrp = hrp;
    }
    fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(String::from(key), String::from(value));
    }
    fn remove_header(&mut self, key: &str) {
        self.headers.remove(key);
    }
    fn remove_all_headers(&mut self) {
        self.headers.clear();
    }
    fn set_auth_token(&mut self, token: &str) {
        self.auth = Some(String::from(token));
    }
    fn get(
        &self,
        url: &str,
        get_data: HashMap<&str, &str>,
        headers: HashMap<&str, &str>,
    ) -> ResponseFuture {
        self.request(url, Method::GET, get_data, Body::empty(), headers)
    }
    fn delete(
        &self,
        url: &str,
        get_data: HashMap<&str, &str>,
        headers: HashMap<&str, &str>,
    ) -> ResponseFuture {
        self.request(url, Method::DELETE, get_data, Body::empty(), headers)
    }
    fn put(&self, url: &str, post_data: Body, headers: HashMap<&str, &str>) -> ResponseFuture {
        self.request(url, Method::PUT, HashMap::new(), post_data, headers)
    }
    fn post(&self, url: &str, post_data: Body, headers: HashMap<&str, &str>) -> ResponseFuture {
        self.request(url, Method::POST, HashMap::new(), post_data, headers)
    }
    fn patch(&self, url: &str, post_data: Body, headers: HashMap<&str, &str>) -> ResponseFuture {
        self.request(url, Method::PATCH, HashMap::new(), post_data, headers)
    }
    fn add_api(&mut self, api_name: &'static str, api: Box<dyn ApiBase>) {
        self.apis.insert(api_name, api);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let avalanche: Result<Avalanche, AvalancheError> =
            Avalanche::new("example.com", 8000, Some("http"), None, None, None, None, false);
        match avalanche {
            Ok(avalanche) => {
                assert_eq!(avalanche.url, "http://example.com:8000");
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn bad_character_in_host() {
        let avalanche: Result<Avalanche, AvalancheError> =
            Avalanche::new("e&&xample.com", 8000, Some("http"), None, None, None, None, false);
        match avalanche {
            Ok(avalanche) => {
                assert_eq!(avalanche.url, "http://example.com:8000");
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn bad_protocol() {
        let avalanche: Result<Avalanche, AvalancheError> =
            Avalanche::new("example.com", 8000, Some("test"), None, None, None, None, false);
        match avalanche {
            Ok(_) => {
                assert!(false);
            }
            Err(_) => {
                assert!(true);
            }
        }
    }
}

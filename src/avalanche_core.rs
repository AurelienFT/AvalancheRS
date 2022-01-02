use crate::errors::{AvalancheError};
use std::collections::HashMap;
use hyper::{Body};
use hyper::client::{ResponseFuture};
use crate::common::api_base::ApiBase;

pub trait AvalancheCore {
    fn set_address(&mut self, host: &'static str, port: u32, protocol: Option<&'static str>) -> Result<(), AvalancheError>;
    fn get_protocol(&self) -> &str;
    fn get_host(&self) -> &str;
    fn get_ip(&self) -> &str;
    fn get_port(&self) -> u32;
    fn get_url(&self) -> &str;
    fn get_headers(&self) -> &HashMap<String, String>;
    fn get_network_id(&self) -> u16;
    fn set_network_id(&mut self, network_id: u16);
    fn get_hrp(&self) -> &str;
    fn set_hrp(&mut self, hrp: &'static str);
    fn set_header(&mut self, key: &str, value: &str);
    fn remove_header(&mut self, key: &str);
    fn remove_all_headers(&mut self);
    fn set_auth_token(&mut self, token: &str);
    fn get(&self, url: &str, get_data: HashMap<&str, &str>, headers: HashMap<&str, &str>) -> ResponseFuture;
    fn delete(&self, url: &str, get_data: HashMap<&str, &str>, headers: HashMap<&str, &str>) -> ResponseFuture;
    fn post(&self, url: &str, post_data: Body, headers: HashMap<&str, &str>) -> ResponseFuture;
    fn put(&self, url: &str, post_data: Body, headers: HashMap<&str, &str>) -> ResponseFuture;
    fn patch(&self, url: &str, post_data: Body, headers: HashMap<&str, &str>) -> ResponseFuture;
    fn add_api(&mut self, api_name: &'static str, api: Box<dyn ApiBase>);
}

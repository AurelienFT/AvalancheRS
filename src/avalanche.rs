use hyper::{Client, Request, Method, Body};
use url::Url;
use hyper::client::{ResponseFuture};
use hyper::header::{HeaderValue, HeaderName};
use crate::errors::{AvalancheError};
use crate::utils::constants::{FALLBACK_HRP, NETWORK_ID_TO_HRP, DEFAULT_NETWORK_ID};
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
struct Avalanche {
    network_id: u16,
    hrp: String,
    protocol: String,
    host: String,
    ip: String,
    port: u32,
    url: String,
    headers: HashMap<String, String>,
    auth: Option<String>
}

trait AvalancheCore {
    fn set_address(&mut self, host: String, port: u32, protocol: Option<&str>) -> Result<(), AvalancheError>;
    fn get_protocol(&self) -> &str;
    fn get_host(&self) -> &str;
    fn get_ip(&self) -> &str;
    fn get_port(&self) -> u32;
    fn get_url(&self) -> &str;
    fn get_headers(&self) -> &HashMap<String, String>;
    fn get_network_id(&self) -> u16;
    fn set_network_id(&mut self, network_id: u16);
    fn get_hrp(&self) -> &str;
    fn set_hrp(&mut self, hrp: &str);
    fn set_header(&mut self, key: &str, value: &str);
    fn remove_header(&mut self, key: &str);
    fn remove_all_headers(&mut self);
    fn set_auth_token(&mut self, token: &str);
    fn get(&self, url: &str, get_data: HashMap<&str, &str>, headers: HashMap<&str, &str>) -> ResponseFuture;
    fn delete(&self, url: &str, get_data: HashMap<&str, &str>, headers: HashMap<&str, &str>) -> ResponseFuture;
    fn post(&self, url: &str, post_data: Body, headers: HashMap<&str, &str>) -> ResponseFuture;
    fn put(&self, url: &str, post_data: Body, headers: HashMap<&str, &str>) -> ResponseFuture;
    fn patch(&self, url: &str, post_data: Body, headers: HashMap<&str, &str>) -> ResponseFuture;
}

impl Avalanche {
  pub fn new(host: String, port: u32, protocol: Option<&str>) -> Result<Avalanche, AvalancheError> {
    let mut avalanche = Avalanche::default();
    match avalanche.set_address(host, port, protocol) {
      Ok(_) => Ok(avalanche),
      Err(e) => Err(e)
    }
  }
  fn set_header(&self, mut request: hyper::Request<hyper::Body>, headers: HashMap<&str, &str>) -> hyper::Request<hyper::Body>{
    for (key, value) in headers {
      request.headers_mut().insert(HeaderName::from_str(key).unwrap(), HeaderValue::from_str(value).unwrap());
    }
    for (key, value) in self.headers.iter() {
      request.headers_mut().insert(HeaderName::from_str(key).unwrap(), HeaderValue::from_str(value).unwrap());
    }
    match self.auth {
      Some(ref token) => {
        request.headers_mut().insert(HeaderName::from_str("Authorization").unwrap(), HeaderValue::from_str(format!("Bearer {}", token)).unwrap());
      },
      None => {}
    }
    request
  }
  fn request(&self, url: &str, method: Method, get_data: HashMap<&str, &str>, post_data: Body, headers: HashMap<&str, &str>) -> ResponseFuture {
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
    Client::new().request(request)
  }
}

fn get_preferred_hrp(network_id: Option<u16>) -> &'static str {
  match network_id {
    Some(network_id) => {
      match NETWORK_ID_TO_HRP.get(&network_id) {
        Some(hrp) => hrp,
        None => NETWORK_ID_TO_HRP.get(&DEFAULT_NETWORK_ID).unwrap()
      }
    },
    None => FALLBACK_HRP
  }
}

impl AvalancheCore for Avalanche {
    fn set_address(&mut self, mut host: String, port: u32, protocol: Option<&str>) -> Result<(), AvalancheError> {
        let re = Regex::new(r"[&#,@+()$~%':*?<>{}]").unwrap(); //TODO: Add "
        host = re.replace_all(&host, "").into_owned();
        let protocol_defined: &str = protocol.unwrap_or("http");
        let protocols: Vec<&str> = vec!["http", "https"];
        if !protocols.contains(&protocol_defined) {
          return Err(AvalancheError);
        }
        self.url = format!("{}://{}:{}", &protocol_defined, &host, &port);
        Ok(())
    }
    fn get_protocol(&self) -> &str {
        &self.protocol
    }
    fn get_host(&self) -> &str {
        &self.host
    }
    fn get_ip(&self) -> &str {
        &self.ip
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
        self.hrp = String::from(get_preferred_hrp(Some(network_id)));
    }
    fn get_hrp(&self) -> &str {
        &self.hrp
    }
    fn set_hrp(&mut self, hrp: &str) {
        self.hrp = String::from(hrp);
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
    fn get(&self, url: &str, get_data: HashMap<&str, &str>, headers: HashMap<&str, &str>) -> ResponseFuture {
      self.request(url, Method::GET, get_data, Body::empty(), headers)
    }
    fn delete(&self, url: &str, get_data: HashMap<&str, &str>, headers: HashMap<&str, &str>) -> ResponseFuture {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let avalanche: Result<Avalanche, AvalancheError> = Avalanche::new(String::from("example.com"), 8000, Some("http"));
        match avalanche {
          Ok(avalanche) => {
            assert_eq!(avalanche.url, "http://example.com:8000");
          },
          Err(_) => {
            assert!(false);
          }
        }
    }

    #[test]
    fn bad_character_in_host() {
        let avalanche: Result<Avalanche, AvalancheError> = Avalanche::new(String::from("e&&xample.com"), 8000, Some("http"));
        match avalanche {
          Ok(avalanche) => {
            assert_eq!(avalanche.url, "http://example.com:8000");
          },
          Err(_) => {
            assert!(false);
          }
        }
    }

    #[test]
    fn bad_protocol() {
      let avalanche: Result<Avalanche, AvalancheError> = Avalanche::new(String::from("example.com"), 8000, Some("test"));
      match avalanche {
        Ok(_) => {
          assert!(false);
        },
        Err(_) => {
          assert!(true);
        }
      }
  }
}
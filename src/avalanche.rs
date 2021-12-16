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


/*
  export default class AvalancheCore {
    protected networkID: number = 0
    protected hrp: string = ""
    protected protocol: string
    protected ip: string
    protected host: string
    protected port: number
    protected url: string
    protected auth: string = undefined
    protected headers: { [k: string]: string } = {}
    protected requestConfig: AxiosRequestConfig = {}
    protected apis: { [k: string]: APIBase } = {}


    /**
     * Adds an API to the middleware. The API resolves to a registered blockchain's RPC.
     *
     * In TypeScript:
     * ```js
     * avalanche.addAPI<MyVMClass>("mychain", MyVMClass, "/ext/bc/mychain")
     * ```
     *
     * In Javascript:
     * ```js
     * avalanche.addAPI("mychain", MyVMClass, "/ext/bc/mychain")
     * ```
     *
     * @typeparam GA Class of the API being added
     * @param apiName A label for referencing the API in the future
     * @param ConstructorFN A reference to the class which instantiates the API
     * @param baseurl Path to resolve to reach the API
     *
     */
    addAPI = <GA extends APIBase>(
      apiName: string,
      ConstructorFN: new (
        avax: AvalancheCore,
        baseurl?: string,
        ...args: any[]
      ) => GA,
      baseurl: string = undefined,
      ...args: any[]
    ) => {
      if (typeof baseurl === "undefined") {
        this.apis[`${apiName}`] = new ConstructorFN(this, undefined, ...args)
      } else {
        this.apis[`${apiName}`] = new ConstructorFN(this, baseurl, ...args)
      }
    }
  
    /**
     * Retrieves a reference to an API by its apiName label.
     *
     * @param apiName Name of the API to return
     */
    api = <GA extends APIBase>(apiName: string): GA =>
      this.apis[`${apiName}`] as GA
  
    /**
     * @ignore
     */
    protected _request = async (
      xhrmethod: Method,
      baseurl: string,
      getdata: object,
      postdata: string | object | ArrayBuffer | ArrayBufferView,
      headers: AxiosRequestHeaders = {},
      axiosConfig: AxiosRequestConfig = undefined
    ): Promise<RequestResponseData> => {
      let config: AxiosRequestConfig
      if (axiosConfig) {
        config = {
          ...axiosConfig,
          ...this.requestConfig
        }
      } else {
        config = {
          baseURL: `${this.protocol}://${this.host}:${this.port}`,
          responseType: "text",
          ...this.requestConfig
        }
      }
      config.url = baseurl
      config.method = xhrmethod
      config.headers = headers
      config.data = postdata
      config.params = getdata
      const resp: AxiosResponse<any> = await axios.request(config)
      // purging all that is axios
      const xhrdata: RequestResponseData = new RequestResponseData(
        resp.data,
        resp.headers,
        resp.status,
        resp.statusText,
        resp.request
      )
      return xhrdata
    }
  
    /**
     * Makes a GET call to an API.
     *
     * @param baseurl Path to the api
     * @param getdata Object containing the key value pairs sent in GET
     * @param headers An array HTTP Request Headers
     * @param axiosConfig Configuration for the axios javascript library that will be the
     * foundation for the rest of the parameters
     *
     * @returns A promise for [[RequestResponseData]]
     */
    get = (
      baseurl: string,
      getdata: object,
      headers: object = {},
      axiosConfig: AxiosRequestConfig = undefined
    ): Promise<RequestResponseData> =>
      this._request(
        "GET",
        baseurl,
        getdata,
        {},
        this._setHeaders(headers),
        axiosConfig
      )
  
    /**
     * Makes a DELETE call to an API.
     *
     * @param baseurl Path to the API
     * @param getdata Object containing the key value pairs sent in DELETE
     * @param headers An array HTTP Request Headers
     * @param axiosConfig Configuration for the axios javascript library that will be the
     * foundation for the rest of the parameters
     *
     * @returns A promise for [[RequestResponseData]]
     */
    delete = (
      baseurl: string,
      getdata: object,
      headers: object = {},
      axiosConfig: AxiosRequestConfig = undefined
    ): Promise<RequestResponseData> =>
      this._request(
        "DELETE",
        baseurl,
        getdata,
        {},
        this._setHeaders(headers),
        axiosConfig
      )
  
    /**
     * Makes a POST call to an API.
     *
     * @param baseurl Path to the API
     * @param getdata Object containing the key value pairs sent in POST
     * @param postdata Object containing the key value pairs sent in POST
     * @param headers An array HTTP Request Headers
     * @param axiosConfig Configuration for the axios javascript library that will be the
     * foundation for the rest of the parameters
     *
     * @returns A promise for [[RequestResponseData]]
     */
    post = (
      baseurl: string,
      getdata: object,
      postdata: string | object | ArrayBuffer | ArrayBufferView,
      headers: object = {},
      axiosConfig: AxiosRequestConfig = undefined
    ): Promise<RequestResponseData> =>
      this._request(
        "POST",
        baseurl,
        getdata,
        postdata,
        this._setHeaders(headers),
        axiosConfig
      )
  
    /**
     * Makes a PUT call to an API.
     *
     * @param baseurl Path to the baseurl
     * @param getdata Object containing the key value pairs sent in PUT
     * @param postdata Object containing the key value pairs sent in PUT
     * @param headers An array HTTP Request Headers
     * @param axiosConfig Configuration for the axios javascript library that will be the
     * foundation for the rest of the parameters
     *
     * @returns A promise for [[RequestResponseData]]
     */
    put = (
      baseurl: string,
      getdata: object,
      postdata: string | object | ArrayBuffer | ArrayBufferView,
      headers: object = {},
      axiosConfig: AxiosRequestConfig = undefined
    ): Promise<RequestResponseData> =>
      this._request(
        "PUT",
        baseurl,
        getdata,
        postdata,
        this._setHeaders(headers),
        axiosConfig
      )
  
    /**
     * Makes a PATCH call to an API.
     *
     * @param baseurl Path to the baseurl
     * @param getdata Object containing the key value pairs sent in PATCH
     * @param postdata Object containing the key value pairs sent in PATCH
     * @param parameters Object containing the parameters of the API call
     * @param headers An array HTTP Request Headers
     * @param axiosConfig Configuration for the axios javascript library that will be the
     * foundation for the rest of the parameters
     *
     * @returns A promise for [[RequestResponseData]]
     */
    patch = (
      baseurl: string,
      getdata: object,
      postdata: string | object | ArrayBuffer | ArrayBufferView,
      headers: object = {},
      axiosConfig: AxiosRequestConfig = undefined
    ): Promise<RequestResponseData> =>
      this._request(
        "PATCH",
        baseurl,
        getdata,
        postdata,
        this._setHeaders(headers),
        axiosConfig
      )
  
    /**
     * Creates a new Avalanche instance. Sets the address and port of the main Avalanche Client.
     *
     * @param host The hostname to resolve to reach the Avalanche Client APIs
     * @param port The port to resolve to reach the Avalanche Client APIs
     * @param protocol The protocol string to use before a "://" in a request, ex: "http", "https", "git", "ws", etc ...
     */
    constructor(host: string, port: number, protocol: string = "http") {
      this.setAddress(host, port, protocol)
    }
  }
*/
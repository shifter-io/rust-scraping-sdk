#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, deny(warnings))]
#![doc(html_root_url = "https://docs.rs/reqwest/0.11.11")]

//! # Shifter WebScraping API
//!
//! Shifter Web Scraping is an API that allows scraping websites while using rotating proxies to prevent bans.
//! This SDK for Rust makes the usage of the API easier to implement in any project you have.
//!
//! Using the SDK it's quite easy.
//! An example of a GET call to the API is the following:
//!
//! ```rust
//! # async fn get_example(wsa: &WebScrapingAPI<'_>) -> Result<(), Box<dyn Error>> {
//! let mut query_builder = QueryBuilder::new();
//!
//! query_builder.url("http://httpbin.org/headers");
//! query_builder.render_js("1");
//!
//! let mut headers: HashMap<String, String> = HashMap::new();
//! headers.insert("Wsa-test".to_string(), "abcd".to_string());
//!
//! query_builder.headers(headers);
//!
//! let html = wsa.get(query_builder).await?.text().await?;
//!
//! println!("{}", html);
//! # Ok(())
//! # }
//! ```
//!
//! As sometimes WebScrapingAPI might update their parameters, it is useful to have a raw_get method as well.
//! This is useful in order to be able to add any new parameters implemented by WebScrapingAPI.
//! Here's how to do a raw_get request.
//! 
//! ```rust
//! # async fn raw_get_example(wsa: &WebScrapingAPI<'_>) -> Result<(), Box<dyn Error>> {
//! let mut params: HashMap<&str, &str> = HashMap::new();
//! params.insert("url", "http://httpbin.org/headers");
//! 
//! let mut headers: HashMap<String, String> = HashMap::new();
//! headers.insert("Wsa-test".to_string(), "abcd".to_string());
//! 
//! let html = wsa.raw_get(params, headers).await?.text().await?;
//!
//! println!("{}", html);
//! # Ok(())
//! # }
//! ```
//! 
//! Also for post requests you can just use raw_post and post.
//! For these methods you will need to add the body (HashMap<&str,&str>).
//! To add the body to the query builder use query_builder.body(body); where body is a HashMap<&str, &str> with your body
//! To add the body to a raw_post request use wsa.raw_post(params, headers, body)

extern crate reqwest;

use std::collections::HashMap;
use urlencoding::encode;
use reqwest::{Response, Client, header::HeaderMap};
use std::error::Error;
use std::convert::TryInto;

/// The query builder struct that contains the params, headers and body of a request
pub struct QueryBuilder<'a> {
    params: HashMap<&'a str, &'a str>,
    headers: HashMap<String, String>,
    body: HashMap<&'a str, &'a str>,
}

impl<'a> QueryBuilder<'a> {
    /// QueryBuilder constructor
    pub fn new() -> QueryBuilder<'a> {
        QueryBuilder { 
            params: HashMap::new(), 
            headers: HashMap::new(),
            body: HashMap::new(),
        }
    }

    /// Returns a clone of the params hashmap
    pub fn get_params(&self) -> HashMap<&str, &str> {
        self.params.clone()
    }

    /// Sets the headers hashmap for the QueryBuilder
    pub fn headers(&mut self, headers: HashMap<String, String>) {
        self.headers = headers;
    }

    /// Returns the headers hashmap for the QueryBuilder
    pub fn get_headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    /// Sets the body hashmap for the QueryBuilder
    pub fn body(&mut self, body: HashMap<&'a str, &'a str>) {
        self.body = body;
    }

    /// Returns the body hashmap for the QueryBuilder
    pub fn get_body(&self) -> HashMap<&str, &str> {
        self.body.clone()
    }

    /// Sets the url parameter for the QueryBuilder
    pub fn url(&mut self, value: &'a str) {
        self.params.insert("url", value);
    }

    /// Returns the url parameter for the QueryBuilder
    pub fn get_url(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("url").unwrap();
        Ok(*result)
    }

    /// Sets the render_js parameter for the QueryBuilder
    pub fn render_js(&mut self, value: &'a str) {
        self.params.insert("render_js", value);
    }

    /// Returns the render_js parameter for the QueryBuilder
    pub fn get_render_js(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("render_js").unwrap();
        Ok(*result)
    }

    /// Sets the proxy_type parameter for the QueryBuilder
    pub fn proxy_type(&mut self, value: &'a str) {
        self.params.insert("proxy_type", value);
    }

    /// Returns the proxy_type parameter for the QueryBuilder
    pub fn get_proxy_type(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("proxy_type").unwrap();
        Ok(*result)
    }

    /// Sets the country parameter for the QueryBuilder
    pub fn country(&mut self, value: &'a str) {
        self.params.insert("country", value);
    }

    /// Returns the country parameter for the QueryBuilder
    pub fn get_country(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("country").unwrap();
        Ok(*result)
    }

    /// Sets the keep_headers parameter for the QueryBuilder
    pub fn keep_headers(&mut self, value: &'a str) {
        self.params.insert("keep_headers", value);
    }

    /// Returns the keep_headers parameter for the QueryBuilder
    pub fn get_keep_headers(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("keep_headers").unwrap();
        Ok(*result)
    }

    /// Sets the session parameter for the QueryBuilder
    pub fn session(&mut self, value: &'a str) {
        self.params.insert("session", value);
    }

    /// Returns the session parameter for the QueryBuilder
    pub fn get_session(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("session").unwrap();
        Ok(*result)
    }

    /// Sets the timeout parameter for the QueryBuilder
    pub fn timeout(&mut self, value: &'a str) {
        self.params.insert("timeout", value);
    }

    /// Returns the timeout parameter for the QueryBuilder
    pub fn get_timeout(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("timeout").unwrap();
        Ok(*result)
    }

    /// Sets the device parameter for the QueryBuilder
    pub fn device(&mut self, value: &'a str) {
        self.params.insert("device", value);
    }

    /// Returns the device parameter for the QueryBuilder
    pub fn get_device(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("device").unwrap();
        Ok(*result)
    }

    /// Sets the wait_until parameter for the QueryBuilder
    pub fn wait_until(&mut self, value: &'a str) {
        self.params.insert("wait_until", value);
    }

    /// Returns the wait_until parameter for the QueryBuilder
    pub fn get_wait_until(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("wait_until").unwrap();
        Ok(*result)
    }

    /// Sets the wait_for parameter for the QueryBuilder
    pub fn wait_for(&mut self, value: &'a str) {
        self.params.insert("wait_for", value);
    }

    /// Returns the wait_for parameter for the QueryBuilder
    pub fn get_wait_for(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("wait_for").unwrap();
        Ok(*result)
    }

    /// Sets the wait_for_css parameter for the QueryBuilder
    pub fn wait_for_css(&mut self, value: &'a str) {
        self.params.insert("wait_for_css", value);
    }

    /// Returns the wait_for_css parameter for the QueryBuilder
    pub fn get_wait_for_css(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("wait_for_css").unwrap();
        Ok(*result)
    }

    /// Sets the screenshot parameter for the QueryBuilder
    pub fn screenshot(&mut self, value: &'a str) {
        self.params.insert("screenshot", value);
    }

    /// Returns the screenshot parameter for the QueryBuilder
    pub fn get_screenshot(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("screenshot").unwrap();
        Ok(*result)
    }

    /// Sets the extract_rules parameter for the QueryBuilder
    pub fn extract_rules(&mut self, value: &'a str) {
        self.params.insert("extract_rules", value);
    }

    /// Returns the extract_rules parameter for the QueryBuilder
    pub fn get_extract_rules(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("extract_rules").unwrap();
        Ok(*result)
    }

    /// Sets the disable_stealth parameter for the QueryBuilder
    pub fn disable_stealth(&mut self, value: &'a str) {
        self.params.insert("disable_stealth", value);
    }

    /// Returns the disable_stealth parameter for the QueryBuilder
    pub fn get_disable_stealth(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("disable_stealth").unwrap();
        Ok(*result)
    }

    /// Sets the auto_parser parameter for the QueryBuilder
    pub fn auto_parser(&mut self, value: &'a str) {
        self.params.insert("auto_parser", value);
    }

    /// Returns the auto_parser parameter for the QueryBuilder
    pub fn get_auto_parser(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("auto_parser").unwrap();
        Ok(*result)
    }

    /// Sets the js_instructions parameter for the QueryBuilder
    pub fn js_instructions(&mut self, value: &'a str) {
        self.params.insert("js_instructions", value);
    }

    /// Returns the js_instructions parameter for the QueryBuilder
    pub fn get_js_instructions(&self) -> Result<&str, Box<dyn Error>> {
        let result = self.params.get("js_instructions").unwrap();
        Ok(*result)
    }
}

/// The WebScrapingAPI client that makes the requests
pub struct WebScrapingAPI<'a> {
    key: &'a str,
    client: Client,
}

impl<'a> WebScrapingAPI<'a> {
    /// The WebScrapingAPI constructor
    pub fn new(api_key: &str) -> WebScrapingAPI {
        WebScrapingAPI {
            key: api_key,
            client: Client::new(),
        }
    }

    /// Parses parameters and encodes them correctly
    fn params_to_api_url(&self, params: HashMap<&str, &str>) -> String {
        let mut query_string: String = String::from("");

        for (key, value) in params.into_iter() {
            let mut final_value = String::from(value);

            if key == "url" {
                final_value = encode(value).into_owned();
            }

            let parameter_query_string = format!("&{}={}", key, final_value);
            query_string.push_str(&parameter_query_string);
        }

        format!("https://scrape.shifter.io/v1?api_key={}{}", self.key, query_string)
    }

    /// WebScrapingAPI get request based on the query builder
    pub async fn get(&self, query_builder: QueryBuilder<'a>) -> Result<Response, Box<dyn Error>> {
        let headers: HeaderMap = (&query_builder.get_headers()).try_into().expect("Invalid headers");
        let api_url = self.params_to_api_url(query_builder.get_params());
        let response = self.client.get(api_url).headers(headers).send().await?;
        Ok(response)
    }

    /// WebScrapingAPI post request based on the query builder
    pub async fn post(&self, query_builder: QueryBuilder<'a>) -> Result<Response, Box<dyn Error>> {
        let headers: HeaderMap = (&query_builder.get_headers()).try_into().expect("Invalid headers");
        let api_url = self.params_to_api_url(query_builder.get_params());
        let response = self.client.post(api_url).json(&query_builder.get_body()).headers(headers).send().await?;
        Ok(response)
    }

    /// WebScrapingAPI put request based on the query builder
    pub async fn put(&self, query_builder: QueryBuilder<'a>) -> Result<Response, Box<dyn Error>> {
        let headers: HeaderMap = (&query_builder.get_headers()).try_into().expect("Invalid headers");
        let api_url = self.params_to_api_url(query_builder.get_params());
        let response = self.client.put(api_url).json(&query_builder.get_body()).headers(headers).send().await?;
        Ok(response)
    }
    
    /// WebScrapingAPI get request based on HashMap raw parameters
    pub async fn raw_get(&self, params: HashMap<&str, &str>, headers: HashMap<String, String>) -> Result<Response, Box<dyn Error>> {
        let headers: HeaderMap = (&headers).try_into().expect("Invalid headers");
        let api_url = self.params_to_api_url(params);
        let response = self.client.get(api_url).headers(headers).send().await?;
        Ok(response)
    }

    /// WebScrapingAPI post request based on HashMap raw parameters
    pub async fn raw_post(&self, params: HashMap<&str, &str>, headers: HashMap<String, String>, body: HashMap<&str, &str>) -> Result<Response, Box<dyn Error>> {
        let headers: HeaderMap = (&headers).try_into().expect("Invalid headers");
        let api_url = self.params_to_api_url(params);
        let response = self.client.post(api_url).json(&body).headers(headers).send().await?;
        Ok(response)
    }

    /// WebScrapingAPI put request based on HashMap raw parameters
    pub async fn raw_put(&self, params: HashMap<&str, &str>, headers: HashMap<String, String>, body: HashMap<&str, &str>) -> Result<Response, Box<dyn Error>> {
        let headers: HeaderMap = (&headers).try_into().expect("Invalid headers");
        let api_url = self.params_to_api_url(params);
        let response = self.client.put(api_url).json(&body).headers(headers).send().await?;
        Ok(response)
    }
}
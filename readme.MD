# Shifter's Web Scraping Rust SDK

Shifter [Web Scraping](https://shifter.io/services/webscraping-api) is an API that allows scraping websites while using rotating proxies to prevent bans. This SDK for Rust makes the usage of the API easier to implement in any project you have.

## Installation

Add the following dependancy:

```
web-scraping = "0.1.0"
```

## API Key

To use the API and the SDK you will need a API Key. You can get one by registering at [Shifter](https://shifter.io/)

## Usage

Using the SDK it's quite easy.
An example of a GET call to the API is the following:

```
use web_scraping::WebScrapingAPI;
use web_scraping::QueryBuilder;
use std::collections::HashMap;
use std::error::Error;

async fn get_example(wsa: &WebScrapingAPI<'_>) ->Result<(), Box<dyn Error>> {
    let mut query_builder = QueryBuilder::new();

    query_builder.url("http://httpbin.org/headers");
    query_builder.render_js("1");

    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Wsa-test".to_string(), "abcd".to_string());

    query_builder.headers(headers);

    let html = wsa.get(query_builder).await?.text().await?;

    println!("{}", html);

    Ok(())
}

async fn raw_get_example(wsa: &WebScrapingAPI<'_>) ->Result<(), Box<dyn Error>> {
    let mut params: HashMap<&str, &str> = HashMap::new();
    params.insert("url", "http://httpbin.org/headers");

    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Wsa-test".to_string(), "abcd".to_string());

    let html = wsa.raw_get(params, headers).await?.text().await?;

    println!("{}", html);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let wsa: WebScrapingAPI = WebScrapingAPI::new("YOUR_API_KEY");

    get_example(&wsa).await?;

    raw_get_example(&wsa).await?;

    Ok(())
}
```

Notice that in order to run the async request for web-scraping from main we used the dependency:

```tokio = { version = "1", features = ["full"] }```

All dependencies of the crate:

```
urlencoding = "2.1.0"
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
```

For a better understanding of the parameters, please check out [our documentation](http://developers.shifter.io/)

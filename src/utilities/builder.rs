use crate::models::products::ClientBuilderOptions;
use anyhow::Result;

use reqwest::{header, header::HeaderMap, Client, Proxy};
use std::time::Duration;

/// Create and return an instance of [reqwest::Client](https://docs.rs/reqwest/latest/reqwest/struct.Client.html)
pub fn initialize(client_builder_options: ClientBuilderOptions) -> Result<Client> {
    let headers = &client_builder_options.headers;
    let mut header_map: HeaderMap = headers.try_into()?;
    header_map.insert(
        "User-Agent",
        // YandexBot user-agent: Mozilla/5.0 (compatible; YandexBot/3.0; +http://yandex.com/bots)
        // header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:90.0)
        // Gecko/20100101 Firefox/90.0"),
        header::HeaderValue::from_static(""),
    );

    let client = Client::builder()
        .timeout(Duration::new(client_builder_options.timeout, 0))
        .gzip(client_builder_options.is_gzip)
        .default_headers(header_map)
        .user_agent(&client_builder_options.user_agent);

    if let Some(some_proxy) = client_builder_options.proxy {
        if !some_proxy.is_empty() {
            // it's not an empty string; set the proxy
            let proxy_obj = Proxy::all(some_proxy)?;
            return Ok(client.proxy(proxy_obj).build()?);
        }
    }

    Ok(client.build()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// create client with a proxy, expect no error
    fn client_with_good_proxy() {
        let _proxy = "http://127.0.0.1:8080";
        initialize(ClientBuilderOptions::default()).unwrap();
    }
}

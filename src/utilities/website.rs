use std::io::Read;

use crate::models::products::{ClientBuilderOptions, CrawlConfig};
use anyhow::Result;
use bytes::Bytes;
use flate2::read::GzDecoder;
use reqwest::header::{HeaderMap, HeaderValue};
use select::{
    document::Document,
    predicate::{Class, Name},
};

/// This function would search for a phrase inside sitemap links and store them in an array.
/// By default there is a built-in user-agent for each requests. There are some other extra-headers
/// Which can activate by putting `extra_headers` parameter to `true`.
pub async fn get_sitemap_links(sitemap: &str, find_it: &str) -> Result<Vec<String>> {
    let conf = CrawlConfig::default();
    let default_client = conf.client;
    let sitemap_response = default_client.get(sitemap).send().await?.text().await?;

    // In this section we create a new array of links which contains `product` keyword
    // in their URL.
    let mut links: Vec<String> = vec![];

    Document::from(sitemap_response.as_str())
        .find(Name("loc"))
        .map(|n| n.text())
        .for_each(|f| {
            if f.contains(find_it) {
                links.push(f)
            }
        });

    Ok(links)
}

pub fn get_sitemap_links_by_content(content: &str, find_it: &str) -> Result<Vec<String>> {
    // In this section we create a new array of links which contains `product` keyword
    // in their URL.
    let mut links: Vec<String> = vec![];

    Document::from(content)
        .find(Name("loc"))
        .map(|n| n.text())
        .for_each(|f| {
            if f.contains(find_it) {
                links.push(f)
            }
        });

    Ok(links)
}

pub async fn get_product_detail(product_link: &str, search_class: &str) -> Result<String> {
    let product_page_response = reqwest::get(product_link.to_string()).await?.text().await?;

    let mut title = String::new();
    Document::from(product_page_response.as_str())
        .find(Class(search_class))
        .for_each(|f| title.push_str(f.text().as_str()));

    Ok(title)
}

// Send a simple request to get site response.

pub async fn get_response(link: &str) -> Result<String> {
    let client_request_options = ClientBuilderOptions::default();

    let mut is_gzip = client_request_options.is_gzip;

    if link.ends_with(".gz") {
        is_gzip = true;
    }

    if is_gzip {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept-Encoding",
            HeaderValue::from_static("gzip, deflate, br"),
        );
        let client = reqwest::Client::builder()
            .gzip(true)
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()?;

        let mut response_string = String::new();
        let response_bytes = client.get(link).send().await?.bytes().await?;
        let mut encoded_gz_response = GzDecoder::new(&*response_bytes);

        encoded_gz_response.read_to_string(&mut response_string)?;

        Ok(response_string)
    } else {
        let client = reqwest::Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()?;
        let response = client.get(link).send().await?.text().await?;

        Ok(response)
    }
}

pub async fn get_bytes(link: &str, conf: &CrawlConfig) -> Result<Bytes> {
    let client = &conf.client;

    let response = client.get(link).send().await?.bytes().await?;

    Ok(response)
}

use crate::{
    models::utils::{threads, timeout},
    utilities::builder,
};
use anyhow::Result;
use heck::AsTitleCase;

use chrono::Utc;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum Currency {
    Euro,
    #[default]
    Dollar,
    Pound,
}

/// It's possible to create a new product item base on a default trait:
/// let sample_product = Product::default();
/// let other_product = Product {name: "something".to_string(), ..Default::default()};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub name: String,
    pub link: String,
    pub price: f64,
    pub currency: Currency,
    pub description: String,
    pub updated_at: String,
    pub tags: Vec<String>,
    pub main_cat_link: String,
    pub enable: bool,
    pub discount: u64,
    pub image: String,
    pub color: String,
    pub brand: String,
}

impl Default for Product {
    fn default() -> Self {
        Product {
            name: String::from(""),
            link: String::new(),
            price: 0.0,
            currency: Currency::Dollar,
            updated_at: Utc::now().date_naive().to_string(),
            tags: vec![],
            main_cat_link: "".to_string(),
            enable: true,
            discount: 0,
            image: "".to_string(),
            color: "".to_string(),
            description: "".to_string(),
            brand: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct ShopDetail {
    pub country: String,
    pub sitename: String,
    pub site_address: String,
    pub sitemap_address: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ShopConfig {
    pub is_gzip: bool,
    pub timeout: u64,
    pub delay: i32,
    // Requests per second.
    pub requests: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Headers {
    pub user_agent: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MetaConfig {
    work_field: Vec<String>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MainConfig {
    pub shop_detail: ShopDetail,
    pub shop_config: ShopConfig,
    pub custom_headers: Option<Headers>,
    pub meta_config: Option<MetaConfig>,
}

#[derive(Debug, Clone)]
pub struct CrawlConfig {
    pub sitename: String,
    pub site_address: String,
    pub sitemap_address: String,
    pub user_agent: String,
    pub headers: HashMap<String, String>,
    pub is_gzip: bool,
    pub gz_sitemap_links: Vec<String>,
    pub timeout: u64,
    pub threads: i8,
    pub client: Client,
}

/// Client Builder
#[derive(Debug)]
pub struct ClientBuilderOptions {
    pub timeout: u64,
    pub user_agent: String,
    pub headers: HashMap<String, String>,
    pub proxy: Option<String>,
    pub is_gzip: bool,
}

use crate::Args;
use clap::Parser;
// Get default value for ClientBuilderOptions from user input on command line and implement `Default` trait for it.

impl Default for ClientBuilderOptions {
    fn default() -> Self {
        let args = Args::parse();
        let timeout = args.timeout.unwrap_or(10);
        let is_gzip = args.gzip.unwrap_or(false);
        let builtin_user_agent = format!(
            "{}/{}",
            AsTitleCase(env!("CARGO_PKG_NAME")),
            env!("CARGO_PKG_VERSION")
        );
        let user_agent = args.user_agent.unwrap_or(builtin_user_agent);

        let proxy = args.proxy;

        ClientBuilderOptions {
            timeout,
            user_agent,
            headers: HashMap::new(),
            proxy,
            is_gzip,
        }
    }
}
/// Check for body response
/// How to check status with check status:
///  let conf  = CrawlConfig{
///  sitemap_address: "https://www.technoworld.com/computing".to_string(),
///  sitename: "technoworld".to_string()
///  };
///  let future_status = conf.check_status();
///  let status = block_on(status).unwrap();

impl CrawlConfig {
    pub async fn check_status(&self) -> Result<StatusCode> {
        let header_response = self
            .client
            .get(&self.sitemap_address)
            .send()
            .await?
            .status();
        Ok(header_response)
    }

    pub async fn response(&self) -> Result<String> {
        let header_response = self
            .client
            .get(&self.sitemap_address)
            .send()
            .await?
            .text()
            .await?;
        Ok(header_response)
    }
}

impl Default for CrawlConfig {
    fn default() -> Self {
        let timeout = timeout();
        let user_agent = format!(
            "{}/{}",
            AsTitleCase(env!("CARGO_PKG_NAME")),
            env!("CARGO_PKG_VERSION")
        );
        let threads = threads();
        let is_gzip = false;
        let gz_sitemap_links = Vec::new();
        let site_address = String::new();
        let sitemap_address = String::new();
        let sitename = String::new();
        let headers = HashMap::new();
        let client = builder::initialize(ClientBuilderOptions::default())
            .expect("Ops! There was an error since building a client");
        CrawlConfig {
            user_agent,
            client,
            headers,
            timeout,
            is_gzip,
            gz_sitemap_links,
            site_address,
            sitemap_address,
            sitename,
            threads,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct CrawlOutput {
    pub shop: ShopDetail,
    pub products: Vec<Product>,
}

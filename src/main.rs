//! Pricy is a web crawler to find products and their price.
//! It crawls shops for products with off and store them in DB.
#![allow(dead_code)]
mod models;
mod shops;
mod utilities;
use crate::shops::ShopName;
use anyhow::Result;
use clap::Parser;
use itertools::Itertools;

use lazy_static::lazy_static;
use strum::VariantNames;
use utilities::url_reader;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Shop names to crawl
    // short and long flags (-S, --shops) will be deduced from the field's name
    #[clap(
        short = 's',
        long = "shops",
        num_args =  0..        // default_value = "technoworld curry"
    )]
    shops_name: Vec<ShopName>,
    /// Limit pages to crawl
    #[clap(long = "limit-products")]
    limit_products: Option<usize>,
    /// Keyword to filter products
    #[clap(short, long = "filter-keyword")]
    filter_keyword: Option<String>,
    /// Filter products by URL
    #[clap(long = "filter-url")]
    filter_url: Option<String>,

    /// List of indexed shops
    #[clap(short = 'l', long = "list")]
    list: bool,
    /// Custom headers
    #[clap(short = 'H', long = "Header")]
    header: Option<String>,
    /// Use a custom user-agent
    #[clap(short = 'u', long = "user-agent")]
    user_agent: Option<String>,
    /// Proxy to use
    /// Example: --proxy "https://127.0.0.1:3001"
    #[clap(long = "proxy")]
    proxy: Option<String>,
    /// Accept urls by piping all URLs.
    #[clap(short = 'p', long = "pipe")]
    pipe: bool,
    /// Crawl multiple products in one request
    #[clap(short = 'm', long = "multiple_products")]
    multiple_products: Option<String>,
    /// Store sitemaps in Storage. Add "--gzip true" if sitemap is in gzip format
    #[clap(long = "save-sitemap")]
    save_sitemap: bool,
    /// Decodes sitemap with gzip content
    #[clap(short = 'g', long = "gzip")]
    gzip: Option<bool>,

    /// Store sitemaps in Storage.
    #[clap(long = "crawl_in_storage_urls")]
    crawl_in_storage_urls: bool,
    /// Database interactions
    #[clap(short = 'd', long = "database")]
    database: bool,
    /// Timeout for requests, Default is 10 seconds
    #[clap(short = 't', long = "timeout", default_value = "10")]
    timeout: Option<u64>,
}

pub struct DynamicArgs {
    pub limit_products: usize,
    pub filter_keyword: String,
    pub filter_url: String,
}

impl Default for DynamicArgs {
    fn default() -> Self {
        let opt = Args::parse();

        let limit_products = opt.limit_products.unwrap_or(1);
        let filter_keyword = opt.filter_keyword.unwrap_or("".to_string());
        Self {
            limit_products,
            filter_keyword,
            filter_url: opt.filter_url.unwrap_or("".to_string()),
        }
    }
}
lazy_static! {
    pub static ref DYNAMIC_ARGS: DynamicArgs = DynamicArgs::default();
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Args::parse();
    for shop in &opt.shops_name {
        match opt {
            Args {
                crawl_in_storage_urls: true,
                ..
            } => {
                shop.to_shop().crawl_in_storage_urls().await?;
                return Ok(());
            }
            Args {
                save_sitemap: true, ..
            } => {
                shop.store_sitemap_urls_in_storage(opt.gzip.is_some())
                    .await?;
                return Ok(());
            }
            _ => {
                if shop.to_shop().can_crawl().await {
                    shop.to_shop().crawl().await?;
                } else {
                    eprint!(
                        "You are not allow to crawl this websites due to its `robots.txt` file."
                    )
                }
            }
        }
    }

    match opt {
        Args { list: true, .. } => {
            let mut i: u8 = 1;
            for shop in ShopName::VARIANTS.iter().sorted() {
                println!("{}: {:?}", i, *shop);
                i += 1;
            }
        }
        Args {
            multiple_products: Some(_),
            ..
        } => {
            for url in opt.multiple_products.clone().unwrap().split(',') {
                let shop = ShopName::from_url(url);

                if shop.is_none() {
                    eprintln!("Couldn't find shop for {url}");
                    continue;
                }
                let shop = shop.unwrap();
                shop.crawl_single_url(url).await?;
            }
        }

        Args { pipe: true, .. } => {
            url_reader::url_loader_from_pipe().await?;
        }

        _ => {}
    }

    Ok(())
}
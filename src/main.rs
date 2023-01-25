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

    /// List of indexed shops
    #[clap(short = 'l', long = "list")]
    list: bool,
    /// Custom headers
    #[clap(short = 'H', long = "Header")]
    header: Option<String>,
    /// Changes current user-agent to a bot.
    #[clap(short = 'b', long = "bot")]
    bot: bool,
    /// Accept urls by piping all URLs.
    #[clap(short = 'p', long = "pipe")]
    pipe: bool,
    /// Store sitemaps in Storage. Add "--gzip true" if sitemap is in gzip format
    #[clap(long = "save_sitemap")]
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
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
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

        Args { pipe: true, .. } => {
            url_reader::url_loader_from_pipe().await?;
        }

        _ => {}
    }

    Ok(())
}

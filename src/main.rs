//! Cheappy is a web crawler to find products and their price.
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
    /// List of store names to crawl
    // short and long flags (-S, --shops) will be deduced from the field's name
    #[clap(
        short = 's',
        long = "shops",
        // multiple_values =  true
        // default_value = "technoworld curry"
    )]
    shops_name: Vec<ShopName>,

    /// List of indexed shops to crawl.
    #[clap(short = 'l', long = "list")]
    list: bool,
    /// List of indexed shops to crawl.
    #[clap(short = 'H', long = "Header")]
    header: Option<String>,
    /// Changes current user-agent to a bot.
    #[clap(short = 'b', long = "bot")]
    bot: bool,
    /// Accept urls by piping all URLs.
    #[clap(short = 'p', long = "pipe")]
    pipe: bool,
    /// Store sitemaps in Storage.
    #[clap(long = "save_sitemap")]
    save_sitemap: bool,

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
        if opt.crawl_in_storage_urls {
            shop.to_shop().crawl_in_storage_urls().await?;
            return Ok(());
        }
        if opt.save_sitemap {
            shop.store_sitemap_urls_in_storage().await?;
            return Ok(());
        }
        if shop.to_shop().can_crawl().await {
            shop.to_shop().crawl().await?;
        } else {
            eprint!("You are not allow to crawl this websites due to its `robots.txt` file.")
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

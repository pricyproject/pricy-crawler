use crate::{
    models::products::{ClientBuilderOptions, CrawlConfig},
    shops::shop_trait::Shop,
    utilities::{
        self,
        conf_loader::config_loader,
        website::{get_response, get_sitemap_links_by_content},
    },
};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use core::fmt::Display;
use heck::ToSnekCase;
use select::document::Document;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CurrysCoUk;
impl CurrysCoUk {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn crawl_single_url(valid_url: &str) -> Result<()> {
        let m = valid_url.to_string();
        println!("{m:?}");
        Ok(())
    }
}
impl Display for CurrysCoUk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[async_trait]
impl Shop for CurrysCoUk {
    async fn crawl(&self) -> Result<()> {
        let _headers = HashMap::from([("Accept-Encoding".to_string(), "gzip".to_string())]);
        let main_config = config_loader(Self.to_string().to_snek_case())?;
        let shop_detail = main_config.shop_detail.clone();
        let shop_config = main_config.shop_config.clone();
        let conf = CrawlConfig {
            sitename: shop_detail.sitename,
            site_address: shop_detail.site_address,
            sitemap_address: shop_detail.sitemap_address,
            timeout: shop_config.timeout,
            user_agent: String::new(),
            client: utilities::builder::initialize(ClientBuilderOptions::default())?,
            is_gzip: shop_config.is_gzip,
            gz_sitemap_links: Vec::new(),
            ..Default::default()
        };

        let response = conf.response().await?;

        println!("{response:?}");
        let links: Vec<String> = get_sitemap_links_by_content(&response, "")?;

        for link in links.iter().take(1) {
            let product_page = get_response(link, false).await?;
            let document = Document::from(product_page.as_str());

            println!("{document:?}");
            // for node in document.find(Name("ProductListItem__DivWrapper-sc-pb4x98-7
            // cgxObq")).take(1){     // let product_name =
            // node.select(Attr("data-product", "name"));     println!("{:?}",node);
            // }
        }

        // let mut scraped_data_vector = Vec::new();
        // for resp in results.iter() {
        //     let mut product_title_brands = Vec::new();
        //     Document::from(resp.as_str())
        //         .select(Class("productTitle").descendant(Name("span")))
        //         .filter_map(|f| f.last_child())
        //         .for_each(|f| product_title_brands.push(f.text()));

        //     // Combine ["brand", "product name", "xbrand", "x product title"] to
        //     // ["brand product name", "x brand product title"]
        //     let product_titles = product_title_brands
        //         .chunks(2)
        //         .map(|f| f.join(" "))
        //         .collect::<Vec<String>>();

        //     let mut product_prices = Vec::new();
        //     Document::from(resp.as_str())
        //         .select(Class("price"))
        //         .filter_map(|f| f.last_child())
        //         .for_each(|f| product_prices.push(f.text().trim().to_string()));

        //     let mut product_links = Vec::new();
        //     Document::from(resp.as_str())
        //         .select(Class("productTitle").descendant(Name("a")))
        //         .filter_map(|f| f.attr("href"))
        //         .for_each(|f| product_links.push(f.to_string()));

        //     // https://stackoverflow.com/questions/29669287/how-can-i-zip-more-than-two-iterators
        //     for (product_title, product_price_and_currency, product_link) in
        //         izip!(product_titles, product_prices, product_links)
        //     {
        //         // First we need to find

        //         // For more detail please check: https://www.dotnetperls.com/find-rust

        // let product_currency = product_price_and_currency
        //     .find(|c: char| c.is_ascii_digit())
        //     .unwrap();
        // let product_price = product_price_and_currency
        //     .get(product_currency..)
        //     .unwrap()
        //     .parse::<f64>()
        //     .unwrap();
        // let product = Product {
        //     ..Default::default()
        // };
        //         scraped_data_vector.push(product);
        //     }
        // }

        // println!("{}", serde_json::to_string(&scraped_data_vector).unwrap());
        Ok(())
    }
    async fn can_crawl(&self) -> bool {
        true
    }
    async fn crawl_in_storage_urls(&self) -> Result<()> {
        Ok(())
    }
}

use crate::{
    models::products::{CrawlOutput, Currency, Product},
    shops::shop_trait::Shop,
    utilities::{conf_loader::config_loader, website::get_response},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use heck::ToSnakeCase;
use select::{document::Document, predicate::Attr};
use std::{fmt::Display, fs::read_to_string};
#[derive(Debug)]
pub struct TronyIt;

impl TronyIt {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn crawl_single_url(valid_url: &str) -> Result<()> {
        let m = valid_url.to_string();
        println!("{m:?}");
        Ok(())
    }
}
impl Display for TronyIt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[async_trait]
impl Shop for TronyIt {
    async fn crawl(&self) -> Result<()> {
        let main_config = config_loader(Self.to_string().to_snake_case())?;
        let sitemap_file_path = format!(
            "sitemaps/{}/plain/{}/sitemap.txt",
            Self.to_string().to_snake_case(),
            Utc::now().date().naive_utc()
        );
        let sitemap_file = read_to_string(sitemap_file_path)?;
        let mut products: Vec<Product> = Vec::new();
        for link in sitemap_file.lines().take(2) {
            let product_page = get_response(link, false).await?;
            let document = Document::from(product_page.as_str());
            let mut product = Product {
                ..Default::default()
            };
            for node in document.find(Attr("type", "application/ld+json")).take(1) {
                let result = node.text();
                let json_result: serde_json::Value = serde_json::from_str(&result)?;
                product.link = link.to_string();
                product.image = json_result["image"].as_str().unwrap().to_string();
                product.name = json_result["name"].as_str().unwrap().to_string();
                product.currency = Currency::Euro;
                product.price = json_result["offers"]["price"]
                    .as_str()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap();
                product.description = json_result["description"].as_str().unwrap().to_string();
                product.brand = json_result["brand"]["name"].as_str().unwrap().to_string();

                products.push(product.clone());
            }
        }
        let crawl_result = CrawlOutput {
            shop: main_config.shop_detail.clone(),
            products,
        };
        println!("{}", serde_json::to_string(&crawl_result)?);
        Ok(())
    }

    async fn can_crawl(&self) -> bool {
        true
    }

    async fn crawl_in_storage_urls(&self) -> Result<()> {
        Ok(())
    }
}

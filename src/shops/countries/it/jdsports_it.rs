use crate::{
    models::products::*,
    shops::shop_trait::Shop,
    utilities::{
        conf_loader::config_loader,
        create_sitemap::create_local_sitemap,
        website::{get_response, get_sitemap_links_by_content},
    },
    DYNAMIC_ARGS,
};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use heck::ToSnakeCase;
use select::{document::Document, predicate::Attr};

use chrono::Utc;
use std::{fmt::Display, fs::read_to_string};
#[derive(Debug)]
pub struct JdsportsIt;

impl JdsportsIt {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn crawl_single_url(valid_url: &str) -> Result<()> {
        let product = crawl_url(valid_url).await?;
        println!("{}", serde_json::to_string_pretty(&product)?);
        Ok(())
    }
}
impl Display for JdsportsIt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

async fn crawl_url(url: &str) -> Result<Product> {
    let product_page_content = get_response(url).await?;
    let mut product = Product {
        ..Default::default()
    };

    if !product_page_content.contains("Nessun prodotto corrisponde alla tua ricerca.")
        && product_page_content.contains(DYNAMIC_ARGS.filter_keyword.as_str())
        && url.contains(DYNAMIC_ARGS.filter_url.as_str())
    {
        let document = Document::from(product_page_content.as_str());

        for node in document.find(Attr("type", "application/ld+json")).take(1) {
            let result = node.text();
            let json_result: serde_json::Value = serde_json::from_str(&result)?;
            product.link = url.to_string();
            product.image = format!("{}.jpg", json_result["image"][0].as_str().unwrap());
            product.name = json_result["name"].as_str().unwrap().to_string();
            product.currency = Currency::Euro;
            product.price = json_result["offers"]["price"]
                .to_string()
                .parse::<f64>()
                .unwrap();
            product.description = json_result["description"].as_str().unwrap().to_string();
            product.brand = json_result["brand"]["name"].as_str().unwrap().to_string();
            product.color = json_result["color"].as_str().unwrap().to_string();
        }
    }

    Ok(product)
}

#[async_trait]
impl Shop for JdsportsIt {
    async fn crawl(&self) -> Result<()> {
        let main_config = config_loader(Self.to_string().to_snake_case())?;
        let shop_detail = main_config.shop_detail;
        let sitemap_file_path = format!(
            "sitemaps/{}/plain/{}/sitemap.txt",
            Self.to_string().to_snake_case(),
            Utc::now().date_naive()
        );
        let sitemap_file_content: String;
        if read_to_string(&sitemap_file_path).is_ok() {
            sitemap_file_content = read_to_string(sitemap_file_path)?;
        } else {
            let sitemap_content = get_response(&shop_detail.sitemap_address).await?;
            let sitemap_links = get_sitemap_links_by_content(sitemap_content.as_str(), "")?;
            sitemap_file_content = sitemap_links
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\n");
            create_local_sitemap(&sitemap_links, false, &shop_detail)?;
        }

        let mut products: Vec<Product> = Vec::new();
        for link in sitemap_file_content.lines().take(5) {
            let product = crawl_url(link).await?;

            products.push(product);
        }
        let crawl_result = CrawlOutput {
            shop: shop_detail.clone(),
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

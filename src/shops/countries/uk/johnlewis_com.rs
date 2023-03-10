use crate::{
    models::products::{ClientBuilderOptions, *},
    shops::shop_trait::Shop,
    utilities::{
        self,
        conf_loader::config_loader,
        website::{get_response, get_sitemap_links, get_sitemap_links_by_content},
    },
    DYNAMIC_ARGS,
};
use anyhow::Result;
use async_trait::async_trait;

use heck::{AsTitleCase, ToSnakeCase};
use robotstxt_with_cache::{DefaultCachingMatcher, DefaultMatcher};
use select::{document::Document, predicate::Attr};

use std::{collections::HashMap, fmt::Display};
#[derive(Debug)]
pub struct JohnlewisCom;

impl JohnlewisCom {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn crawl_single_url(valid_url: &str) -> Result<()> {
        let product = crawl_url(valid_url).await?;
        println!("{}", serde_json::to_string_pretty(&product)?);
        Ok(())
    }
}
impl Display for JohnlewisCom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

async fn crawl_url(url: &str) -> Result<Product> {
    let product_page_content = get_response(url).await?;
    let mut product = Product {
        ..Default::default()
    };

    if !product_page_content.contains("No longer available online")
        && product_page_content.contains(DYNAMIC_ARGS.filter_keyword.as_str())
        && url.contains(DYNAMIC_ARGS.filter_url.as_str())
    {
        let document = Document::from(product_page_content.as_str());

        for node in document.find(Attr("type", "application/ld+json")).take(1) {
            let result = node.text();

            let json_result: serde_json::Value = serde_json::from_str(&result)?;
            product.link = url.to_string();
            let image = format!("{}?.jpg", json_result["image"].to_string().replace('"', ""));
            product.image = image;
            product.currency = Currency::Pound;
            product.name = json_result["name"].to_string().replace('"', "");

            if json_result["offers"]
                .as_object()
                .unwrap()
                .contains_key("lowPrice")
            {
                product.price = json_result["offers"]["lowPrice"]
                    .to_string()
                    .replace('\"', "")
                    .parse::<f64>()
                    .unwrap()
            } else {
                product.price = json_result["offers"]["price"]
                    .to_string()
                    .replace('\"', "")
                    .parse::<f64>()
                    .unwrap()
            }
        }
    }

    Ok(product)
}

#[async_trait]
impl Shop for JohnlewisCom {
    async fn crawl(&self) -> Result<()> {
        let main_config = config_loader(Self.to_string().to_snake_case())?;
        let shop_detail = main_config.shop_detail.clone();
        let shop_config = main_config.shop_config;
        let _custom_headers = main_config.custom_headers.unwrap();
        let headers = HashMap::from([("Accept-Encoding".to_string(), "gzip".to_string())]);
        let mut conf = CrawlConfig {
            sitename: shop_detail.sitename.clone(),
            site_address: shop_detail.site_address,
            sitemap_address: shop_detail.sitemap_address,
            threads: 10,
            timeout: shop_config.timeout,
            user_agent: String::new(),
            // Sitemap is located at: https://www.johnlewis.com/robots.txt
            // Main sitemap address is: https://www.johnlewis.com/siteindex.xml
            // sitemap_address: "https://www.johnlewis.com/sitemap.xml".to_string(),
            headers: headers.clone(),

            client: utilities::builder::initialize(ClientBuilderOptions::default())?,
            is_gzip: true,
            gz_sitemap_links: Vec::new(),
        };
        conf.gz_sitemap_links = get_sitemap_links(&conf.sitemap_address, "products").await?;
        let mut product_links: Vec<String> = vec![];
        for link in conf.gz_sitemap_links.iter().take(1) {
            let content = get_response(link).await?;

            let site_links = get_sitemap_links_by_content(&content.clone(), "")?;

            // extend moves value inside a vector to another vector.
            product_links.extend(site_links);
        }
        let mut products: Vec<Product> = vec![];
        for product_link in product_links.iter().take(DYNAMIC_ARGS.limit_products) {
            let crawled_product: Product = crawl_url(product_link).await?;

            if !crawled_product.name.is_empty() {
                products.push(crawled_product.clone());
            }
        }

        let crawl_result = CrawlOutput {
            shop: main_config.shop_detail.clone(),
            products,
        };
        print!("{}", serde_json::to_string(&crawl_result)?);

        Ok(())
    }
    async fn can_crawl(&self) -> bool {
        let mut matcher = DefaultCachingMatcher::new(DefaultMatcher::default());
        let response = reqwest::get("https://www.johnlewis.com/robots.txt")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        matcher.parse(response.as_str());

        matcher.one_agent_allowed_by_robots(
            AsTitleCase(env!("CARGO_PKG_NAME")).to_string().as_str(),
            "https://www.johnlewis.com/",
        )
    }
    async fn crawl_in_storage_urls(&self) -> Result<()> {
        Ok(())
    }
}

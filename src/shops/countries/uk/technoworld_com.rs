use crate::{models::products::*, shops::shop_trait::Shop, utilities::conf_loader::config_loader};
use anyhow::Result;
use async_trait::async_trait;

use heck::ToSnekCase;
use itertools::izip;
use select::{
    document::Document,
    predicate::{Attr, Class, Name, Predicate},
};
use std::fmt::Display;

#[derive(Debug)]
pub struct TechnoworldCom;

impl TechnoworldCom {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn crawl_single_url(valid_url: &str) -> Result<()> {
        let m = valid_url.to_string();
        let content = reqwest::get(&m).await?.text().await?;
        let document = Document::from(content.as_str());
        for node in document.find(Class("page-main")) {
            println!("{node:?}");
        }

        Ok(())
    }
}
impl Display for TechnoworldCom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[async_trait]
impl Shop for TechnoworldCom {
    async fn crawl(&self) -> Result<()> {
        let config: MainConfig = config_loader(Self.to_string().to_snek_case())?;
        let conf = CrawlConfig {
            sitename: config.shop_detail.sitename,
            site_address: config.shop_detail.site_address,
            sitemap_address: config.shop_detail.sitemap_address,
            ..Default::default()
        };
        let mut product_prices = vec![];
        let mut product_links = vec![];
        let mut product_names = vec![];
        let mut product_images = vec![];
        let content = conf.response().await?;
        let document = Document::from(content.as_str());

        for node in document.find(Attr("class", "product-image-photo")) {
            product_images.push(node.attr("src").unwrap())
        }
        for node in document.find(
            Class("product__wrapper")
                .descendant(Name("div"))
                .descendant(Class("product-item-name")),
        ) {
            let product_name_untrimed = node
                .find(Name("a"))
                .next()
                .unwrap()
                .first_child()
                .unwrap()
                .text();
            product_links.push(
                node.find(Name("a"))
                    .next()
                    .unwrap()
                    .attr("href")
                    .unwrap()
                    .to_string(),
            );
            product_names.push(product_name_untrimed.trim().to_string());
        }

        for node in document.find(
            Class("product__wrapper")
                .descendant(Name("div"))
                .descendant(Class("price")),
        ) {
            let mut price_string = node.first_child().unwrap().text().replace(',', "");
            price_string.remove(0);
            let price = price_string.parse::<f64>().unwrap();

            product_prices.push(price)
        }

        let mut products: Vec<Product> = vec![];
        for (product_image, product_name, product_price, product_link) in
            izip!(product_images, product_names, product_prices, product_links)
        {
            let product = Product {
                name: product_name,
                link: product_link,
                price: product_price,
                currency: Currency::Euro,
                image: product_image.to_string(),
                ..Default::default()
            };
            products.push(product)
        }

        let shop_detail: ShopDetail = config_loader(Self.to_string().to_snek_case())?.shop_detail;
        let crawl_result = CrawlOutput {
            shop: shop_detail,
            products,
        };
        print!("{}", serde_json::to_string(&crawl_result)?);

        Ok(())
    }
    async fn can_crawl(&self) -> bool {
        true
    }
    async fn crawl_in_storage_urls(&self) -> Result<()> {
        Ok(())
    }
}

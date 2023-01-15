use crate::{
    models::products::*,
    shops::shop_trait::Shop,
    utilities::{
        conf_loader::config_loader,
        website::{get_response, get_sitemap_links, get_sitemap_links_by_content},
    },
};
use anyhow::Result;
use async_trait::async_trait;
use heck::ToSnakeCase;
use select::document::Document;
use std::fmt::Display;
#[derive(Debug)]
pub struct NordicnestCom;

impl NordicnestCom {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn crawl_single_url(valid_url: &str) -> Result<()> {
        let m = valid_url.to_string();
        println!("{m:?}");
        Ok(())
    }
}
impl Display for NordicnestCom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}
// First we need to extract all urls from: `https://www.laptopsdirect.co.uk/sitemaps/sitemap-categories.xml`
// Then store that link and crawl inside it.
#[async_trait]
impl Shop for NordicnestCom {
    async fn crawl(&self) -> Result<()> {
        let main_config = config_loader(Self.to_string().to_snake_case())?;
        let _shop_detail = main_config.shop_detail.clone();
        let conf = CrawlConfig {
            site_address: "https://www.nordicnest.com/".to_string(),
            sitemap_address: "https://www.nordicnest.com/api/sitemap/en-us/sitemapindex.xml"
                .to_string(),

            ..Default::default()
        };
        let response = conf.response().await?;
        let sitemaps = get_sitemap_links_by_content(&response, "").unwrap();
        let product_links = get_sitemap_links(sitemaps[1].as_str(), "").await?;
        for link in product_links.iter().take(1) {
            let result = get_response(link, false).await?;
            let document = Document::from(result.as_str());
            // for node in document.find(Attr()) {
            //     println!("{:?}", node)
            // }
            println!("{document:?}")
        }
        Ok(())
    }

    async fn can_crawl(&self) -> bool {
        true
    }
    async fn crawl_in_storage_urls(&self) -> Result<()> {
        Ok(())
    }
}

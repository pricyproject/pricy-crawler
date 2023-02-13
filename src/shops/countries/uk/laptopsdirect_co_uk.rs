use crate::{
    models::products::{ClientBuilderOptions, *},
    shops::shop_trait::Shop,
    utilities::{
        self, builder,
        conf_loader::config_loader,
        website::{get_response, get_sitemap_links_by_content},
    },
};
use anyhow::Result;
use async_trait::async_trait;
use heck::ToSnakeCase;
use select::{document::Document, predicate::Attr};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub struct LaptopsdirectCoUk;

impl LaptopsdirectCoUk {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn crawl_single_url(valid_url: &str) -> Result<()> {
        let m = valid_url.to_string();
        println!("{m:?}");
        Ok(())
    }
}

impl Display for LaptopsdirectCoUk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}
// First we need to extract all urls from: `https://www.laptopsdirect.co.uk/sitemaps/sitemap-categories.xml`
// Then store that link and crawl inside it.
#[async_trait]
impl Shop for LaptopsdirectCoUk {
    async fn crawl(&self) -> Result<()> {
        let _tags: Vec<_> = vec!["Computer", "Laptop", "Electronic"];
        let main_config = config_loader(Self.to_string().to_snake_case())?;
        let _shop_detail = main_config.shop_detail.clone();
        let mut config = CrawlConfig {
            // In this case main site map is: `https://www.laptopsdirect.co.uk/sitemaps/sitemap-index.xml`
            // We are trying to put `https://www.laptopsdirect.co.uk/sitemaps/sitemap-categories.xml` as an alternative.
            sitename: "laptopsdirect.co.uk".to_string(),
            site_address: "https://www.laptopsdirect.co.uk/".to_string(),
            sitemap_address: "https://www.laptopsdirect.co.uk/sitemaps/sitemap-categories.xml"
                .to_string(),
            client: utilities::builder::initialize(ClientBuilderOptions::default())?,
            ..Default::default()
        };

        println!("wow");
        let _new_header: HashMap<String, String> = HashMap::new();
        config.client = builder::initialize(ClientBuilderOptions::default())?;
        let content = config.response().await?;
        let links = get_sitemap_links_by_content(&content, "")?;

        for i in links.iter().take(1) {
            let link = i.as_str();
            let contents = get_response(link).await?;
            let document = Document::from(contents.as_str());
            for node in document.find(Attr("id", "products")) {
                print!("{node:?}")
            }
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

pub mod countries;

pub mod shop_trait;

use crate::{
    shops::{
        countries::{
            it::{jdsports_it::JdsportsIt, trony_it::TronyIt},
            uk::{
                currys_co_uk::CurrysCoUk, johnlewis_com::JohnlewisCom,
                laptopsdirect_co_uk::LaptopsdirectCoUk, nordicnest_com::NordicnestCom,
                technoworld_com::TechnoworldCom,
            },
        },
        shop_trait::Shop,
    },
    utilities::{
        conf_loader::config_loader,
        create_sitemap::create_local_sitemap,
        website::{get_response, get_sitemap_links_by_content},
    },
};
use anyhow::Result;
use strum::{Display, EnumString, EnumVariantNames};

#[derive(EnumString, Debug, EnumVariantNames, Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum ShopName {
    TechnoworldCom,
    CurrysCoUk,
    LaptopsdirectCoUk,
    JohnlewisCom,
    NordicnestCom,
    JdsportsIt,
    TronyIt,
}

impl ShopName {
    pub fn to_shop(&self) -> Box<dyn Shop> {
        match self {
            ShopName::TechnoworldCom => Box::new(TechnoworldCom::new()),
            ShopName::CurrysCoUk => Box::new(CurrysCoUk::new()),
            ShopName::LaptopsdirectCoUk => Box::new(LaptopsdirectCoUk::new()),
            ShopName::JohnlewisCom => Box::new(JohnlewisCom::new()),
            ShopName::JdsportsIt => Box::new(JdsportsIt::new()),
            ShopName::NordicnestCom => Box::new(NordicnestCom::new()),
            ShopName::TronyIt => Box::new(TronyIt::new()),
        }
    }

    pub async fn crawl_single_url<'a>(&'a self, url: &'a str) -> Result<()> {
        match self {
            // UK shops
            ShopName::TechnoworldCom => TechnoworldCom::crawl_single_url(url).await,
            ShopName::CurrysCoUk => CurrysCoUk::crawl_single_url(url).await,
            ShopName::LaptopsdirectCoUk => LaptopsdirectCoUk::crawl_single_url(url).await,
            ShopName::JohnlewisCom => JohnlewisCom::crawl_single_url(url).await,
            ShopName::NordicnestCom => NordicnestCom::crawl_single_url(url).await,
            // Shops in Italy
            ShopName::JdsportsIt => JdsportsIt::crawl_single_url(url).await,
            ShopName::TronyIt => TronyIt::crawl_single_url(url).await,
        }
    }

    pub async fn store_sitemap_urls_in_storage(&self) -> Result<()> {
        let main_config = config_loader(self.to_string())?;
        let shop_detail = main_config.shop_detail.clone();
        let sitemap_content = get_response(&shop_detail.sitemap_address, true).await?;
        let sitemap_links = get_sitemap_links_by_content(sitemap_content.as_str(), "")?;
        create_local_sitemap(&sitemap_links, false, &shop_detail)?;

        Ok(())
    }
}

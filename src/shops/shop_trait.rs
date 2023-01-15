use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Shop {
    async fn crawl(&self) -> Result<()>;
    async fn can_crawl(&self) -> bool;
    async fn crawl_in_storage_urls(&self) -> Result<()>;
}

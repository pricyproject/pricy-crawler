use anyhow::Result;
use async_trait::async_trait;
use crate::iinodels::products::CrawlConfig;
#[async_trait]
pub trait Request{
    async fn response(&self) -> Result<()>;
}
    

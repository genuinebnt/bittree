use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;

use crate::domain::{
    entities::{block::Block, page::Page},
    errors::Result,
    types::{BlockId, PageId, WorkspaceId},
};

#[async_trait]
pub trait PageRepository: Send + Sync + 'static + Debug {
    async fn create(&self, page: Page) -> Result<PageId>;
    async fn find_by_id(&self, id: PageId) -> Result<Page>;
    async fn find_by_workspace(&self, workspace_id: WorkspaceId) -> Result<Vec<Page>>;
    async fn update(&self, page: Page) -> Result<()>;
    async fn soft_delete(&self, id: PageId) -> Result<()>;
}

pub type DynPageRepository = Arc<dyn PageRepository>;

#[async_trait]
pub trait BlockRepository: Send + Sync + 'static + Debug {
    async fn create(&self, block: Block) -> Result<BlockId>;
    async fn find_by_id(&self, id: BlockId) -> Result<Block>;
    async fn find_by_page(&self, page_id: PageId) -> Result<Vec<Block>>;
    async fn update_content(
        &self,
        id: BlockId,
        content: serde_json::Value,
        expected_version: i32,
    ) -> Result<()>;
    async fn soft_delete(&self, id: BlockId) -> Result<()>;
}

pub type DynBlockRepository = Arc<dyn BlockRepository>;

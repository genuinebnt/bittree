// PageRepository trait (async_trait, Send + Sync + 'static + Debug):
//   create(page: Page) -> Result<PageId>
//   find_by_id(id: PageId) -> Result<Page>
//   find_by_workspace(workspace_id: WorkspaceId) -> Result<Vec<Page>>
//   update(page: Page) -> Result<()>
//   soft_delete(id: PageId) -> Result<()>
//
// pub type DynPageRepository = Arc<dyn PageRepository>
//
// BlockRepository trait (async_trait, Send + Sync + 'static + Debug):
//   create(block: Block) -> Result<BlockId>
//   find_by_id(id: BlockId) -> Result<Block>
//   find_by_page(page_id: PageId) -> Result<Vec<Block>>
//   update_content(id: BlockId, content: Value, expected_version: i32) -> Result<()>
//   soft_delete(id: BlockId) -> Result<()>
//
// pub type DynBlockRepository = Arc<dyn BlockRepository>

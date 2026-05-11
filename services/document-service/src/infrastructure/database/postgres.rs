// PostgresPageRepository { pool: PgPool }
//   impl PageRepository — sqlx queries against docs.pages
//   PageRow (sqlx::FromRow) + TryFrom<PageRow> for Page
//
// PostgresBlockRepository { pool: PgPool }
//   impl BlockRepository — sqlx queries against docs.blocks
//   BlockRow (sqlx::FromRow) + TryFrom<BlockRow> for Block
//   update_content uses optimistic locking: WHERE id=$1 AND version=$2
//   returns DomainError::VersionConflict if 0 rows affected

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::errors::{DomainError, Result};
use crate::domain::types::{DateTimeWithTimezone, UserId, WorkspaceId};
use crate::domain::{entities::page::Page, repository::PageRepository, types::PageId};

#[derive(Debug, sqlx::FromRow)]
pub struct PageRow {
    id: Uuid,
    workspace_id: Uuid,
    parent_id: Option<Uuid>,
    created_by: Uuid,
    last_edited_by: Uuid,
    title: String,
    icon: Option<String>,
    cover_url: Option<String>,
    is_database: bool,
    visibility: String,
    locked: bool,
    locked_by: Option<Uuid>,
    version: i32,
    published_slug: Option<String>,
    created_at: DateTimeWithTimezone,
    updated_at: DateTimeWithTimezone,
    deleted_at: Option<DateTimeWithTimezone>,
}

impl TryFrom<PageRow> for Page {
    type Error = DomainError;

    fn try_from(value: PageRow) -> std::result::Result<Self, DomainError> {
        Ok(Self::from_parts(
            value.id.into(),
            value.workspace_id.into(),
            value.parent_id.map(|id| id.into()),
            value.created_by.into(),
            value.last_edited_by.into(),
            value.title,
            value.icon,
            value.cover_url,
            value.is_database,
            value.visibility.try_into()?,
            value.locked,
            value.locked_by.map(|id| id.into()),
            value.version,
            value.published_slug,
            value.created_at,
            value.updated_at,
            value.deleted_at,
        ))
    }
}

#[derive(Debug)]
pub struct PostgresPageRepository {
    pub pool: PgPool,
}

#[async_trait]
impl PageRepository for PostgresPageRepository {
    async fn create(&self, page: Page) -> Result<PageId> {
        let page_id = sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO docs.pages(
                id,
                workspace_id,
                parent_id,
                created_by,
                last_edited_by,
                title,
                icon,
                cover_url,
                is_database,
                visibility,
                locked,
                locked_by,
                version,
                published_slug,
                created_at,
                updated_at,
                deleted_at
            )
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7,
                $8,
                $9,
                $10,
                $11,
                $12,
                $13,
                $14,
                $15,
                $16,
                $17
            )
            RETURNING id;",
        )
        .bind(*page.id().as_ref())
        .bind(*page.workspace_id().as_ref())
        .bind(page.parent_id().map(|id| *id.as_ref()))
        .bind(*page.created_by().as_ref())
        .bind(*page.last_edited_by().as_ref())
        .bind(page.title())
        .bind(page.icon())
        .bind(page.cover_url())
        .bind(page.is_database())
        .bind(page.visibility().as_str())
        .bind(page.locked())
        .bind(page.locked_by().map(|id| *id.as_ref()))
        .bind(page.version())
        .bind(page.published_slug())
        .bind(page.created_at())
        .bind(page.updated_at())
        .bind(page.deleted_at())
        .fetch_one(&self.pool)
        .await?;

        Ok(page_id.into())
    }
    async fn find_by_id(&self, id: PageId) -> Result<Page> {
        let row = sqlx::query_as::<_, PageRow>(
            "SELECT
                id,
                workspace_id,
                parent_id,
                created_by,
                last_edited_by,
                title,
                icon,
                cover_url,
                is_database,
                visibility,
                locked,
                locked_by,
                version,
                published_slug,
                created_at,
                updated_at,
                deleted_at
            FROM docs.pages
            WHERE id = $1
            LIMIT 1",
        )
        .bind(id.as_ref())
        .fetch_optional(&self.pool)
        .await?;

        row.ok_or(DomainError::PageNotFound(id.to_string()))
            .and_then(|row| row.try_into())
    }
    async fn find_by_workspace(&self, workspace_id: WorkspaceId) -> Result<Vec<Page>> {
        let rows = sqlx::query_as::<_, PageRow>(
            "SELECT
                id,
                workspace_id,
                parent_id,
                created_by,
                last_edited_by,
                title,
                icon,
                cover_url,
                is_database,
                visibility,
                locked,
                locked_by,
                version,
                published_slug,
                created_at,
                updated_at,
                deleted_at
            FROM docs.pages
            WHERE workspace_id = $1",
        )
        .bind(workspace_id.as_ref())
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| row.try_into())
            .collect::<std::result::Result<Vec<Page>, _>>()
    }
    async fn update(&self, page: Page) -> Result<()> {
        let result = sqlx::query(
            "
            UPDATE docs.pages
                SET
                    parent_id = $1,
                    last_edited_by = $2,
                    title = $3,
                    icon = $4,
                    cover_url = $5,
                    is_database = $6,
                    visibility = $7,
                    locked = $8,
                    locked_by = $9,
                    version = version + 1,
                    published_slug = $10,
                    updated_at = $11,
                    deleted_at = $12
                WHERE id = $13
                    AND version = $14
        ",
        )
        .bind(page.parent_id().map(|id| *id.as_ref()))
        .bind(*page.last_edited_by().as_ref())
        .bind(page.title())
        .bind(page.icon())
        .bind(page.cover_url())
        .bind(page.is_database())
        .bind(page.visibility().as_str())
        .bind(page.locked())
        .bind(page.locked_by().map(|id| *id.as_ref()))
        .bind(page.published_slug())
        .bind(page.updated_at())
        .bind(page.deleted_at())
        .bind(page.id().as_ref())
        .bind(page.version())
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(DomainError::PageNotFound(page.id().to_string()));
        }

        Ok(())
    }
    async fn soft_delete(&self, id: PageId) -> Result<()> {
        let result = sqlx::query(
            "
            UPDATE docs.pages
                SET
                    deleted_at = NOW()
            WHERE id = $1
                AND deleted_at IS NULL
            ",
        )
        .bind(id.as_ref())
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(DomainError::PageNotFound(id.to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::types::UserId;

    use super::*;

    #[sqlx::test]
    async fn create_page_returns_id(pool: PgPool) {
        sqlx::migrate!().run(&pool).await.unwrap();
        let repo = PostgresPageRepository { pool };
        let workspace_id = WorkspaceId::generate();
        let user_id = UserId::generate();
        let page = Page::new(workspace_id, None, user_id, "Test Page".to_string()).unwrap();
        let expected_id = *page.id();

        let result = repo.create(page).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_id);
    }

    #[sqlx::test]
    async fn find_by_id_returns_created_page(pool: PgPool) {
        sqlx::migrate!().run(&pool).await.unwrap();
        let repo = PostgresPageRepository { pool };
        let workspace_id = WorkspaceId::generate();
        let user_id = UserId::generate();
        let page = Page::new(workspace_id, None, user_id, "Hello".to_string()).unwrap();
        let id = *page.id();
        repo.create(page).await.unwrap();

        let found = repo.find_by_id(id).await.unwrap();

        assert_eq!(*found.id(), id);
        assert_eq!(found.title(), "Hello");
        assert_eq!(*found.workspace_id(), workspace_id);
        assert_eq!(*found.created_by(), user_id);
        assert_eq!(*found.last_edited_by(), user_id);
        assert!(!found.is_deleted());
    }

    #[sqlx::test]
    async fn find_by_id_returns_not_found_for_unknown_id(pool: PgPool) {
        sqlx::migrate!().run(&pool).await.unwrap();
        let repo = PostgresPageRepository { pool };
        let unknown_id = PageId::generate();

        let result = repo.find_by_id(unknown_id).await;

        assert!(matches!(result, Err(DomainError::PageNotFound(_))));
    }

    #[sqlx::test]
    async fn find_by_workspace_returns_all_pages(pool: PgPool) {
        sqlx::migrate!().run(&pool).await.unwrap();
        let repo = PostgresPageRepository { pool };
        let workspace_id = WorkspaceId::generate();
        let user_id = UserId::generate();
        let page_a = Page::new(workspace_id, None, user_id, "Page A".to_string()).unwrap();
        let page_b = Page::new(workspace_id, None, user_id, "Page B".to_string()).unwrap();
        repo.create(page_a).await.unwrap();
        repo.create(page_b).await.unwrap();

        let pages = repo.find_by_workspace(workspace_id).await.unwrap();

        assert_eq!(pages.len(), 2);
    }

    #[sqlx::test]
    async fn find_by_workspace_returns_empty_for_unknown_workspace(pool: PgPool) {
        sqlx::migrate!().run(&pool).await.unwrap();
        let repo = PostgresPageRepository { pool };
        let unknown_workspace = WorkspaceId::generate();

        let pages = repo.find_by_workspace(unknown_workspace).await.unwrap();

        assert!(pages.is_empty());
    }

    #[sqlx::test]
    async fn find_by_workspace_excludes_other_workspaces(pool: PgPool) {
        sqlx::migrate!().run(&pool).await.unwrap();
        let repo = PostgresPageRepository { pool };
        let workspace_a = WorkspaceId::generate();
        let workspace_b = WorkspaceId::generate();
        let user_id = UserId::generate();
        repo.create(Page::new(workspace_a, None, user_id, "A".to_string()).unwrap())
            .await
            .unwrap();
        repo.create(Page::new(workspace_b, None, user_id, "B".to_string()).unwrap())
            .await
            .unwrap();

        let pages = repo.find_by_workspace(workspace_a).await.unwrap();

        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].title(), "A");
    }

    #[sqlx::test]
    async fn soft_delete_sets_deleted_at(pool: PgPool) {
        sqlx::migrate!().run(&pool).await.unwrap();
        let repo = PostgresPageRepository { pool };
        let page = Page::new(
            WorkspaceId::generate(),
            None,
            UserId::generate(),
            "To Delete".to_string(),
        )
        .unwrap();
        let id = *page.id();
        repo.create(page).await.unwrap();

        repo.soft_delete(id).await.unwrap();

        let found = repo.find_by_id(id).await.unwrap();
        assert!(found.is_deleted());
    }

    #[sqlx::test]
    async fn soft_delete_returns_not_found_for_unknown_id(pool: PgPool) {
        sqlx::migrate!().run(&pool).await.unwrap();
        let repo = PostgresPageRepository { pool };

        let result = repo.soft_delete(PageId::generate()).await;

        assert!(matches!(result, Err(DomainError::PageNotFound(_))));
    }
}

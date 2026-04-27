// PostgresPageRepository { pool: PgPool }
//   impl PageRepository — sqlx queries against docs.pages
//   PageRow (sqlx::FromRow) + TryFrom<PageRow> for Page
//
// PostgresBlockRepository { pool: PgPool }
//   impl BlockRepository — sqlx queries against docs.blocks
//   BlockRow (sqlx::FromRow) + TryFrom<BlockRow> for Block
//   update_content uses optimistic locking: WHERE id=$1 AND version=$2
//   returns DomainError::VersionConflict if 0 rows affected

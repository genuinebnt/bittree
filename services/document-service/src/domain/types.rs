// PageId(Uuid), BlockId(Uuid), WorkspaceId(Uuid), UserId(Uuid) — define_id! newtypes
// BlockType — enum: Paragraph, HeadingOne..Three, BulletedListItem, NumberedListItem,
//   Toggle, Quote, Callout, Code, Image, File, Bookmark, Equation, Embed,
//   ColumnList, Column, SyncedBlock, TableOfContents, Breadcrumb, Divider,
//   Database, DatabaseRow — serde rename_all = "snake_case"
// Visibility — enum: Private, Workspace, Custom, Public
// DateTimeWithTimezone = DateTime<Utc>

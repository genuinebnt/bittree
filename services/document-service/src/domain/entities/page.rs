use chrono::Utc;
use getset::Getters;

use crate::domain::{
    errors::{DomainError, Result},
    types::{DateTimeWithTimezone, PageId, UserId, Visibility, WorkspaceId},
};

// Page — maps to docs.pages table
#[derive(Debug, Getters)]
pub struct Page {
    #[getset(get = "pub")]
    id: PageId,
    #[getset(get = "pub")]
    workspace_id: WorkspaceId,
    #[getset(get = "pub")]
    parent_id: Option<PageId>,
    #[getset(get = "pub")]
    created_by: UserId,
    #[getset(get = "pub")]
    last_edited_by: UserId,
    #[getset(get = "pub")]
    title: String,
    #[getset(get = "pub")]
    icon: Option<String>,
    #[getset(get = "pub")]
    cover_url: Option<String>,
    #[getset(get = "pub")]
    is_database: bool,
    #[getset(get = "pub")]
    visibility: Visibility,
    #[getset(get = "pub")]
    locked: bool,
    #[getset(get = "pub")]
    locked_by: Option<UserId>,
    #[getset(get = "pub")]
    version: i32,
    #[getset(get = "pub")]
    published_slug: Option<String>,
    #[getset(get = "pub")]
    created_at: DateTimeWithTimezone,
    #[getset(get = "pub")]
    updated_at: DateTimeWithTimezone,
    #[getset(get = "pub")]
    deleted_at: Option<DateTimeWithTimezone>,
}

impl Page {
    pub fn new(
        workspace_id: WorkspaceId,
        parent_id: Option<PageId>,
        created_by: UserId,
        title: String,
    ) -> Result<Self> {
        let title = title.trim().to_string();
        if title.is_empty() {
            return Err(DomainError::InvalidTitle(title));
        }

        let now = Utc::now();

        Ok(Self {
            id: PageId::generate(),
            workspace_id,
            parent_id,
            created_by,
            last_edited_by: created_by,
            title,
            icon: None,
            cover_url: None,
            is_database: false,
            visibility: Visibility::Workspace,
            locked: false,
            locked_by: None,
            version: 0,
            published_slug: None,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        })
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    pub fn from_parts(
        id: PageId,
        workspace_id: WorkspaceId,
        parent_id: Option<PageId>,
        created_by: UserId,
        last_edited_by: UserId,
        title: String,
        icon: Option<String>,
        cover_url: Option<String>,
        is_database: bool,
        visibility: Visibility,
        locked: bool,
        locked_by: Option<UserId>,
        version: i32,
        published_slug: Option<String>,
        created_at: DateTimeWithTimezone,
        updated_at: DateTimeWithTimezone,
        deleted_at: Option<DateTimeWithTimezone>,
    ) -> Self {
        Self {
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
            deleted_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        errors::DomainError,
        types::{PageId, UserId, Visibility, WorkspaceId},
    };

    fn workspace_id() -> WorkspaceId {
        WorkspaceId::generate()
    }

    fn user_id() -> UserId {
        UserId::generate()
    }

    #[test]
    fn new_page_with_valid_title_succeeds() {
        let page = Page::new(workspace_id(), None, user_id(), "My First Page".to_string());
        assert!(page.is_ok());
        assert_eq!(page.unwrap().title, "My First Page");
    }

    #[test]
    fn new_page_title_is_trimmed() {
        let page = Page::new(workspace_id(), None, user_id(), "  Trimmed  ".to_string()).unwrap();
        assert_eq!(page.title, "Trimmed");
    }

    #[test]
    fn new_page_with_empty_title_fails() {
        let err = Page::new(workspace_id(), None, user_id(), "".to_string()).unwrap_err();
        assert!(matches!(err, DomainError::InvalidTitle(_)));
    }

    #[test]
    fn new_page_with_whitespace_only_title_fails() {
        let err = Page::new(workspace_id(), None, user_id(), "   ".to_string()).unwrap_err();
        assert!(matches!(err, DomainError::InvalidTitle(_)));
    }

    #[test]
    fn new_page_starts_at_version_zero() {
        let page = Page::new(workspace_id(), None, user_id(), "Test".to_string()).unwrap();
        assert_eq!(page.version, 0);
    }

    #[test]
    fn new_page_is_not_deleted() {
        let page = Page::new(workspace_id(), None, user_id(), "Test".to_string()).unwrap();
        assert!(!page.is_deleted());
        assert!(page.deleted_at.is_none());
    }

    #[test]
    fn new_page_is_not_locked() {
        let page = Page::new(workspace_id(), None, user_id(), "Test".to_string()).unwrap();
        assert!(!page.locked);
        assert!(page.locked_by.is_none());
    }

    #[test]
    fn new_page_is_not_a_database() {
        let page = Page::new(workspace_id(), None, user_id(), "Test".to_string()).unwrap();
        assert!(!page.is_database);
    }

    #[test]
    fn new_page_default_visibility_is_workspace() {
        let page = Page::new(workspace_id(), None, user_id(), "Test".to_string()).unwrap();
        assert_eq!(page.visibility, Visibility::Workspace);
    }

    #[test]
    fn new_page_with_parent_records_parent_id() {
        let parent = PageId::generate();
        let page = Page::new(workspace_id(), Some(parent), user_id(), "Child".to_string()).unwrap();
        assert_eq!(page.parent_id, Some(parent));
    }

    #[test]
    fn new_root_page_has_no_parent_id() {
        let page = Page::new(workspace_id(), None, user_id(), "Root".to_string()).unwrap();
        assert!(page.parent_id.is_none());
    }

    #[test]
    fn created_by_and_last_edited_by_match_on_creation() {
        let uid = user_id();
        let page = Page::new(workspace_id(), None, uid, "Test".to_string()).unwrap();
        assert_eq!(page.created_by, uid);
        assert_eq!(page.last_edited_by, uid);
    }

    #[test]
    fn new_page_has_no_published_slug() {
        let page = Page::new(workspace_id(), None, user_id(), "Test".to_string()).unwrap();
        assert!(page.published_slug.is_none());
    }

    #[test]
    fn created_at_and_updated_at_are_equal_on_creation() {
        let page = Page::new(workspace_id(), None, user_id(), "Test".to_string()).unwrap();
        assert_eq!(page.created_at, page.updated_at);
    }
}

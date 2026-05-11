use chrono::Utc;
use serde_json::json;

use crate::domain::{
    errors::{DomainError, Result},
    types::{BlockId, BlockType, DateTimeWithTimezone, PageId, UserId},
};

// Block — maps to docs.blocks table
#[derive(Debug)]
pub struct Block {
    id: BlockId,
    page_id: PageId,
    parent_id: Option<BlockId>,
    created_by: UserId,
    last_edited_by: UserId,
    block_type: BlockType,
    content: serde_json::Value,
    properties: serde_json::Value,
    sort_key: String,
    source_block_id: Option<BlockId>,
    version: i32,
    created_at: DateTimeWithTimezone,
    updated_at: DateTimeWithTimezone,
    deleted_at: Option<DateTimeWithTimezone>,
}

impl Block {
    pub fn new(
        page_id: PageId,
        parent_id: Option<BlockId>,
        created_by: UserId,
        block_type: BlockType,
        sort_key: String,
    ) -> Result<Self> {
        if sort_key.is_empty() {
            return Err(DomainError::InvalidSortKey(sort_key));
        }

        let now = Utc::now();

        Ok(Self {
            id: BlockId::generate(),
            page_id,
            parent_id,
            created_by,
            last_edited_by: created_by,
            block_type,
            content: json!({}),
            properties: json!({}),
            sort_key,
            source_block_id: None,
            version: 0,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        })
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        errors::DomainError,
        types::{BlockId, BlockType, PageId, UserId},
    };
    use serde_json::json;

    fn page_id() -> PageId {
        PageId::generate()
    }

    fn user_id() -> UserId {
        UserId::generate()
    }

    #[test]
    fn new_block_with_valid_args_succeeds() {
        let result = Block::new(
            page_id(),
            None,
            user_id(),
            BlockType::Paragraph,
            "a0".to_string(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn new_block_with_empty_sort_key_fails() {
        let err = Block::new(
            page_id(),
            None,
            user_id(),
            BlockType::Paragraph,
            "".to_string(),
        )
        .unwrap_err();
        assert!(matches!(err, DomainError::InvalidSortKey(_)));
    }

    #[test]
    fn new_block_starts_at_version_zero() {
        let block = Block::new(
            page_id(),
            None,
            user_id(),
            BlockType::Paragraph,
            "a0".to_string(),
        )
        .unwrap();
        assert_eq!(block.version, 0);
    }

    #[test]
    fn new_block_is_not_deleted() {
        let block = Block::new(
            page_id(),
            None,
            user_id(),
            BlockType::Paragraph,
            "a0".to_string(),
        )
        .unwrap();
        assert!(!block.is_deleted());
        assert!(block.deleted_at.is_none());
    }

    #[test]
    fn new_block_has_empty_content() {
        let block = Block::new(
            page_id(),
            None,
            user_id(),
            BlockType::Paragraph,
            "a0".to_string(),
        )
        .unwrap();
        assert_eq!(block.content, json!({}));
    }

    #[test]
    fn new_block_has_empty_properties() {
        let block = Block::new(
            page_id(),
            None,
            user_id(),
            BlockType::Paragraph,
            "a0".to_string(),
        )
        .unwrap();
        assert_eq!(block.properties, json!({}));
    }

    #[test]
    fn new_block_with_no_parent_is_direct_page_child() {
        let block = Block::new(
            page_id(),
            None,
            user_id(),
            BlockType::Paragraph,
            "a0".to_string(),
        )
        .unwrap();
        assert!(block.parent_id.is_none());
    }

    #[test]
    fn new_block_with_parent_records_parent_id() {
        let parent = BlockId::generate();
        let block = Block::new(
            page_id(),
            Some(parent),
            user_id(),
            BlockType::Toggle,
            "a0".to_string(),
        )
        .unwrap();
        assert_eq!(block.parent_id, Some(parent));
    }

    #[test]
    fn created_by_and_last_edited_by_match_on_creation() {
        let uid = user_id();
        let block =
            Block::new(page_id(), None, uid, BlockType::Paragraph, "a0".to_string()).unwrap();
        assert_eq!(block.created_by, uid);
        assert_eq!(block.last_edited_by, uid);
    }

    #[test]
    fn new_block_has_no_source_block() {
        let block = Block::new(
            page_id(),
            None,
            user_id(),
            BlockType::Paragraph,
            "a0".to_string(),
        )
        .unwrap();
        assert!(block.source_block_id.is_none());
    }

    #[test]
    fn new_block_records_its_block_type() {
        let block = Block::new(
            page_id(),
            None,
            user_id(),
            BlockType::Code,
            "a0".to_string(),
        )
        .unwrap();
        assert_eq!(block.block_type, BlockType::Code);
    }

    #[test]
    fn new_block_records_its_sort_key() {
        let block = Block::new(
            page_id(),
            None,
            user_id(),
            BlockType::Paragraph,
            "a0V".to_string(),
        )
        .unwrap();
        assert_eq!(block.sort_key, "a0V");
    }

    #[test]
    fn created_at_and_updated_at_are_equal_on_creation() {
        let block = Block::new(
            page_id(),
            None,
            user_id(),
            BlockType::Paragraph,
            "a0".to_string(),
        )
        .unwrap();
        assert_eq!(block.created_at, block.updated_at);
    }
}

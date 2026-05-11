// PageId(Uuid), BlockId(Uuid), WorkspaceId(Uuid), UserId(Uuid) — define_id! newtypes
// BlockType — enum: Paragraph, HeadingOne..Three, BulletedListItem, NumberedListItem,
//   Toggle, Quote, Callout, Code, Image, File, Bookmark, Equation, Embed,
//   ColumnList, Column, SyncedBlock, TableOfContents, Breadcrumb, Divider,
//   Database, DatabaseRow — serde rename_all = "snake_case"
// Visibility — enum: Private, Workspace, Custom, Public
// DateTimeWithTimezone = DateTime<Utc>

use chrono::{DateTime, Utc};
use infra::define_id;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::errors::{DomainError, Result};

pub type DateTimeWithTimezone = DateTime<Utc>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BlockType {
    Paragraph,
    HeadingOne,
    HeadingTwo,
    HeadingThree,
    BulletedListItem,
    NumberedListItem,
    Toggle,
    Quote,
    Callout,
    Code,
    Image,
    File,
    Bookmark,
    Equation,
    Embed,
    ColumnList,
    Column,
    SyncedBlock,
    TableOfContents,
    Breadcrumb,
    Divider,
    Database,
    DatabaseRow,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Private,
    Workspace,
    Custom,
    Public,
}

impl Visibility {
    pub fn as_str(&self) -> &str {
        match self {
            Visibility::Private => "private",
            Visibility::Workspace => "workspace",
            Visibility::Custom => "custom",
            Visibility::Public => "public",
        }
    }

    pub fn try_from_str(value: &str) -> Result<Self> {
        match value {
            "private" => Ok(Visibility::Private),
            "workspace" => Ok(Visibility::Workspace),
            "custom" => Ok(Visibility::Custom),
            "public" => Ok(Visibility::Public),
            _ => Err(DomainError::VisibilityNotFound(value.to_string())),
        }
    }
}

impl TryFrom<String> for Visibility {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self> {
        Self::try_from_str(&value)
    }
}

impl From<Visibility> for String {
    fn from(value: Visibility) -> Self {
        match value {
            Visibility::Private => "private".to_string(),
            Visibility::Workspace => "workspace".to_string(),
            Visibility::Custom => "custom".to_string(),
            Visibility::Public => "public".to_string(),
        }
    }
}

define_id!(Page);
define_id!(Workspace);
define_id!(User);
define_id!(Block);

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::timestamp::{Timestamp, context::NoContext};

    fn v7() -> Uuid {
        Uuid::new_v7(Timestamp::now(NoContext))
    }

    // ── ID newtypes ──────────────────────────────────────────────────────────

    #[test]
    fn generated_page_ids_are_monotonically_ordered() {
        let first = PageId::generate();
        let second = PageId::generate();
        assert!(
            first.0 < second.0,
            "v7 IDs must be monotonically increasing"
        );
    }

    #[test]
    fn page_id_and_block_id_share_inner_uuid_but_are_distinct_types() {
        let raw = v7();
        let page_id: PageId = PageId(raw);
        let block_id: BlockId = BlockId(raw);
        assert_eq!(page_id.0, raw);
        assert_eq!(block_id.0, raw);
    }

    #[test]
    fn page_id_display_matches_inner_uuid() {
        let raw = v7();
        assert_eq!(PageId(raw).to_string(), raw.to_string());
    }

    #[test]
    fn block_id_display_matches_inner_uuid() {
        let raw = v7();
        assert_eq!(BlockId(raw).to_string(), raw.to_string());
    }

    #[test]
    fn workspace_id_display_matches_inner_uuid() {
        let raw = v7();
        assert_eq!(WorkspaceId(raw).to_string(), raw.to_string());
    }

    #[test]
    fn user_id_display_matches_inner_uuid() {
        let raw = v7();
        assert_eq!(UserId(raw).to_string(), raw.to_string());
    }

    #[test]
    fn page_id_serializes_as_uuid_string() {
        let raw = v7();
        let json = serde_json::to_string(&PageId(raw)).unwrap();
        assert_eq!(json, format!("\"{raw}\""));
    }

    #[test]
    fn page_id_round_trips_through_json() {
        let raw = v7();
        let original = PageId(raw);
        let json = serde_json::to_string(&original).unwrap();
        let back: PageId = serde_json::from_str(&json).unwrap();
        assert_eq!(original, back);
    }

    // ── BlockType serialisation ──────────────────────────────────────────────

    #[test]
    fn block_type_paragraph_serializes_to_snake_case() {
        assert_eq!(
            serde_json::to_string(&BlockType::Paragraph).unwrap(),
            r#""paragraph""#
        );
    }

    #[test]
    fn block_type_heading_one_serializes_to_snake_case() {
        assert_eq!(
            serde_json::to_string(&BlockType::HeadingOne).unwrap(),
            r#""heading_one""#
        );
    }

    #[test]
    fn block_type_bulleted_list_item_serializes_to_snake_case() {
        assert_eq!(
            serde_json::to_string(&BlockType::BulletedListItem).unwrap(),
            r#""bulleted_list_item""#
        );
    }

    #[test]
    fn block_type_database_row_serializes_to_snake_case() {
        assert_eq!(
            serde_json::to_string(&BlockType::DatabaseRow).unwrap(),
            r#""database_row""#
        );
    }

    #[test]
    fn all_block_type_variants_round_trip_through_json() {
        let variants = [
            BlockType::Paragraph,
            BlockType::HeadingOne,
            BlockType::HeadingTwo,
            BlockType::HeadingThree,
            BlockType::BulletedListItem,
            BlockType::NumberedListItem,
            BlockType::Toggle,
            BlockType::Quote,
            BlockType::Callout,
            BlockType::Code,
            BlockType::Image,
            BlockType::File,
            BlockType::Bookmark,
            BlockType::Equation,
            BlockType::Embed,
            BlockType::ColumnList,
            BlockType::Column,
            BlockType::SyncedBlock,
            BlockType::TableOfContents,
            BlockType::Breadcrumb,
            BlockType::Divider,
            BlockType::Database,
            BlockType::DatabaseRow,
        ];
        for variant in variants {
            let json = serde_json::to_string(&variant).unwrap();
            let back: BlockType = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back, "round-trip failed for {json}");
        }
    }

    #[test]
    fn unknown_block_type_string_fails_deserialization() {
        let result: serde_json::Result<BlockType> = serde_json::from_str(r#""not_a_real_type""#);
        assert!(result.is_err());
    }

    // ── Visibility serialisation ─────────────────────────────────────────────

    #[test]
    fn visibility_variants_serialize_to_snake_case() {
        assert_eq!(
            serde_json::to_string(&Visibility::Private).unwrap(),
            r#""private""#
        );
        assert_eq!(
            serde_json::to_string(&Visibility::Workspace).unwrap(),
            r#""workspace""#
        );
        assert_eq!(
            serde_json::to_string(&Visibility::Custom).unwrap(),
            r#""custom""#
        );
        assert_eq!(
            serde_json::to_string(&Visibility::Public).unwrap(),
            r#""public""#
        );
    }

    #[test]
    fn all_visibility_variants_round_trip_through_json() {
        for variant in [
            Visibility::Private,
            Visibility::Workspace,
            Visibility::Custom,
            Visibility::Public,
        ] {
            let json = serde_json::to_string(&variant).unwrap();
            let back: Visibility = serde_json::from_str(&json).unwrap();
            assert_eq!(variant, back, "round-trip failed for {json}");
        }
    }

    #[test]
    fn unknown_visibility_string_fails_deserialization() {
        let result: serde_json::Result<Visibility> = serde_json::from_str(r#""superadmin""#);
        assert!(result.is_err());
    }
}

// Block — maps to docs.blocks table
//   id: BlockId, page_id: PageId, parent_id: Option<BlockId>,
//   created_by: UserId, last_edited_by: UserId, block_type: BlockType,
//   content: serde_json::Value, properties: serde_json::Value,
//   sort_key: String, source_block_id: Option<BlockId>, version: i32,
//   created_at / updated_at / deleted_at: DateTimeWithTimezone

use crate::domain::types::{DateTimeWithTimezone, Email, UserId, Username};

#[derive(Debug)]
pub struct User {
    pub id: UserId,
    pub username: Username,
    pub email: Email,
    pub password_hash: String,
    pub email_verified: bool,
    pub is_active: bool,
    pub created_at: DateTimeWithTimezone,
    pub updated_at: DateTimeWithTimezone,
}

use chrono::{DateTime, Utc};
use infra::{define_id, define_type};
use uuid::Uuid;

use crate::domain::errors::DomainError;
use validator::ValidateEmail;

define_id!(UserId);

impl UserId {
    pub fn new(uuid: Uuid) -> UserId {
        UserId(uuid)
    }
}

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl AsRef<Uuid> for UserId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

define_id!(TokenId);
define_type!(Username, String);

impl Username {
    pub fn parse(s: String) -> Result<Self, DomainError> {
        if !s.len() > 0 {
            return Err(DomainError::InvalidUsername(s));
        }

        Ok(Self(s))
    }
}

impl TryFrom<String> for Username {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::parse(value).unwrap())
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

define_type!(Email, String);

impl Email {
    pub fn parse(s: String) -> Result<Self, DomainError> {
        if !s.validate_email() {
            return Err(DomainError::InvalidEmail(s));
        }

        Ok(Self(s))
    }
}

impl TryFrom<String> for Email {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::parse(value).unwrap())
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub type DateTimeWithTimezone = DateTime<Utc>;

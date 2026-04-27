use secrecy::SecretBox;

use crate::{
    domain::{errors::DomainError, types::Email},
    presentation::handlers::register::RegisterRequest,
};

#[derive(Debug)]
pub struct RegisterCommand {
    pub email: Email,
    pub password: SecretBox<String>,
}

impl TryFrom<RegisterRequest> for RegisterCommand {
    type Error = DomainError;

    fn try_from(request: RegisterRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            email: Email::parse(request.email)?,
            password: request.password,
        })
    }
}

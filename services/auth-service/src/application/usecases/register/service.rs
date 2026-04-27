use super::command::RegisterCommand;
use crate::domain::errors::Result;

pub struct RegisterResponse {}

pub trait RegisterService {
    async fn register(&self, cmd: RegisterCommand) -> Result<RegisterResponse>;
}

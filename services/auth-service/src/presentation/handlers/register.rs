use axum::{Json, extract::State};
use secrecy::SecretBox;
use serde::Deserialize;

use crate::{
    application::commands::register_command::RegisterCommand, presentation::state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: SecretBox<String>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterResponse {}

// register(State, Json<RegisterRequest>) -> Result<Json<AuthResponse>>
pub async fn register_handler(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, String> {
    let regiser_command = RegisterCommand::from(request);
    todo!("")
}

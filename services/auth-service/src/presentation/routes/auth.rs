use axum::Router;

use crate::presentation::{handlers::register::register_handler, state::AppState};

// auth_router(state: Arc<AppState>) -> Router
// POST /register, POST /login, POST /refresh
pub fn auth_router(state: std::sync::Arc<AppState>) -> Router {
    Router::new()
        .route("/signup", register_handler)
        .with_state(state)
}

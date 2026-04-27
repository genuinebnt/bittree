use axum::Router;

use crate::presentation::state::AppState;

// create_router(state: AppState) -> Router
// merges health_check_router, nests auth_router at /auth
pub fn create_router(state: AppState) -> Router {
    Router::new().nest("/auth", auth_router).with_state(state)
}

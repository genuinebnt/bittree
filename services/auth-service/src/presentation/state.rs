// AppState — holds RepoProvider, jwt keys, argon2 config
#[derive(Debug, Clone)]
pub struct AppState {}

impl AppState {
    pub fn new() -> AppState {
        Self {}
    }
}

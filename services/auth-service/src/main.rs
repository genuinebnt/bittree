use auth_service::{
    config::AuthServiceSettings, infrastructure::http::create_router, presentation::state::AppState,
};
use infra::{
    config::get_configuration,
    telemetry::{LogFormat, get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("auth_service", "info", std::io::stdout, LogFormat::Pretty);
    init_subscriber(subscriber);

    let settings = get_configuration::<AuthServiceSettings>("config").unwrap();
    let state = AppState::new();
    let router = create_router(state);

    let address = format!(
        "{}:{}",
        settings.application.host, settings.application.port
    );
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

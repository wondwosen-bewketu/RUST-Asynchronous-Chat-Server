use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize application logging
pub fn init_logger() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_axum_project=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
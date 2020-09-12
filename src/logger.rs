use tracing_subscriber::EnvFilter;

pub fn init_logger() {
    init_tracer(None);
}

pub fn init_tracer(level: Option<tracing::Level>) {
    let _ = tracing_subscriber::fmt()
        .with_max_level(level.unwrap_or(tracing::Level::DEBUG))
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
}

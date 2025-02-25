/// Initialize logger
/// Uses env filter from default env
/// Example
/// ```bash
/// export RUST_LOG=info
/// ```
pub fn init_logger() -> tracing_subscriber::EnvFilter {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    tracing_subscriber::EnvFilter::from_default_env()
}

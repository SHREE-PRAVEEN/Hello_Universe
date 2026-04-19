use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
 
pub fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn,hyper=warn"));
 
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_target(true).with_thread_ids(false).compact())
        .init();
}
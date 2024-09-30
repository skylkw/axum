use admin::constant::CONFIG;
use admin::error::AppResult;
use admin::server::worker::MessengerTask;
use admin::server::AppServer;
use admin::{configure, util};
use futures::FutureExt;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> AppResult<()> {
    let _file_appender_guard = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("The initialization of Tracing was successful.");
    let config = CONFIG.clone();
    info!("Reading the config file was successful.");
    let _sentry_guard = configure::sentry::init(&config.sentry);
    info!("The initialization of Sentry was successful.");
    info!("Create a new server.");
    let server = AppServer::new(config).await?;
    info!("Create a new messenger task.");
    let messenger = MessengerTask::new(server.state.clone());
    info!("Run the server.");
    util::task::join_all(vec![
        (true, server.run().boxed()),
        (true, messenger.run().boxed()),
    ])
    .await?;
    Ok(())
}

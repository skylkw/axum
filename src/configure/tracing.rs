use crate::error::AppResult;
use tracing::{subscriber, Subscriber};
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::{time::LocalTime, MakeWriter};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use time::macros::format_description;



fn create_subscriber<W>(env_filter: EnvFilter, writer: W) -> impl Subscriber + Sync + Send
where
    W: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
  let timer =  LocalTime::new(format_description!("[hour]:[minute]:[second]"));
    Registry::default()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(writer)
                .with_ansi(false)
                .with_timer(timer.clone() ),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout)
                .with_timer(timer),
        )
}

pub fn init_subscriber<S>(subscriber: S) -> anyhow::Result<()>
where
    S: Subscriber + Send + Sync + 'static,
{
    LogTracer::init()?;
    subscriber::set_global_default(subscriber)?;
    Ok(())
}

pub fn init() -> AppResult<WorkerGuard> {
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");
    let (file_appender, file_appender_guard) = tracing_appender::non_blocking(file_appender);
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,axum=debug,tower_http=trace"));
    init_subscriber(create_subscriber(env_filter, file_appender))?;
    Ok(file_appender_guard)
}

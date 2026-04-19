use tracing::subscriber;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, EnvFilter};

pub fn init_tracing(env_filter: String) {
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "nanowave");
    let filter = EnvFilter::new(
        env_filter
    );

    let subscriber = fmt()
        .with_writer(file_appender)
        .with_env_filter(filter)
        .with_ansi(false)
        .finish();

    subscriber::set_global_default(subscriber)
        .expect("Failed to set global subscriber");

    /*
    trace!("trace test");
    debug!("debug test");
    info!("info test");
    warn!("warn test");
    error!("error test");

     */
}

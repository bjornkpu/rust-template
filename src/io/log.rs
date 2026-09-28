use std::path::Path;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;

use crate::error::AppError;

/// Logs to `<dir>/rust-template.log` when `RUST_TEMPLATE_LOG` is set, else not at all.
/// Keep the guard alive until exit so the last lines are flushed.
pub fn init(dir: &Path) -> Result<Option<WorkerGuard>, AppError> {
    let Ok(filter) = std::env::var("RUST_TEMPLATE_LOG") else {
        return Ok(None);
    };
    let filter = EnvFilter::try_new(filter)?;
    std::fs::create_dir_all(dir)?;
    let (writer, guard) =
        tracing_appender::non_blocking(tracing_appender::rolling::never(dir, "rust-template.log"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(writer)
        .with_ansi(false)
        .init();
    Ok(Some(guard))
}

//! The one error type below `main`.

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("RUST_TEMPLATE_LOG is not a valid filter: {0}")]
    LogFilter(#[from] tracing_subscriber::filter::ParseError),
    #[error("no home directory; set RUST_TEMPLATE_HOME")]
    NoHome,
}

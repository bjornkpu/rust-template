//! Everything that touches the outside world. Nothing in `domain` may call into here.

pub mod log;
#[cfg(feature = "tui")]
pub mod terminal;

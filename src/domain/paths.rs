//! Where the tool keeps its files.

use std::path::PathBuf;

use crate::error::AppError;

/// `RUST_TEMPLATE_HOME` when set, else `~/.config/rust-template`.
// ponytail: one dir for config, logs and data; split into XDG config/state/data if a tool needs it.
pub fn home(env_home: Option<PathBuf>, user_home: Option<PathBuf>) -> Result<PathBuf, AppError> {
    if let Some(dir) = env_home {
        return Ok(dir);
    }
    let user_home = user_home.ok_or(AppError::NoHome)?;
    Ok(user_home.join(".config").join("rust-template"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_home_wins() {
        let got = home(Some("/tmp/x".into()), Some("/home/bk".into())).unwrap();
        assert_eq!(got, PathBuf::from("/tmp/x"));
    }

    #[test]
    fn falls_back_to_config_dir_under_user_home() {
        let got = home(None, Some("/home/bk".into())).unwrap();
        assert_eq!(got, PathBuf::from("/home/bk/.config/rust-template"));
    }

    #[test]
    fn no_home_at_all_is_an_error() {
        assert!(matches!(home(None, None), Err(AppError::NoHome)));
    }
}

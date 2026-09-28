//! The hello-world behaviour. /bootstrap replaces this module.

/// The greeting for `name`; a blank name greets the world.
#[must_use]
pub fn greeting(name: &str) -> String {
    let name = name.trim();
    if name.is_empty() {
        "hello, world".to_owned()
    } else {
        format!("hello, {name}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_by_name() {
        assert_eq!(greeting("BK"), "hello, BK");
    }

    #[test]
    fn blank_name_greets_world() {
        assert_eq!(greeting("  "), "hello, world");
    }
}

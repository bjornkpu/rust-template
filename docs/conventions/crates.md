# Crates

The chosen crate for each job, and why. `Cargo.toml` carries the short form of this list; this
file is the authority when they disagree. Uncomment a crate when a task needs it. Never add an
alternative for a job a listed crate does. Ask BK before adding anything not listed.

Versions are pinned to the major only (`"1"`, `"0.30"`); `Cargo.lock` holds the exact version.

## On by default

| Job | Crate | Why |
| --- | --- | --- |
| Error type | `thiserror` | One `AppError` enum with derived messages and `#[from]`. |
| Error context in `main` | `anyhow` | `.context()` at the boundary; never below `main`. |
| Command line | `clap` (`derive`) | Derive keeps arguments, help and parsing in one struct. |
| Logging | `tracing`, `tracing-appender`, `tracing-subscriber` (`env-filter`) | Structured logs to a file, level set at runtime via `RUST_TEMPLATE_LOG`. |
| Snapshot tests (dev) | `insta` | Output, and TUI screens, reviewed as text. |
| Temp dirs (dev) | `tempfile` | Isolated `RUST_TEMPLATE_HOME` per test. |

## Core (commented until first use, uncomment without asking)

| Job | Crate | Why |
| --- | --- | --- |
| (De)serialisation | `serde` (`derive`), `serde_json`, `toml` | The standard; `toml` for config, JSON for data in and out. |
| Dates and times | `jiff` (`serde`) | Correct time zones and spans, a clear API. |

They are commented out only so `cargo machete` passes until the first use.

## Catalog (commented until needed)

| Job | Crate | Rule |
| --- | --- | --- |
| TUI rendering | `ratatui` | TUI only. The maintained TUI library; `TestBackend` makes screens snapshot-testable. |
| TUI events | `crossterm` | TUI only, imported in `src/io/terminal.rs` alone. ratatui's default backend; works on Windows. |
| HTTP | `reqwest` (`json`, `query`) | Only behind a boundary trait with a fake. |
| Async runtime | `tokio` (`rt-multi-thread`, `macros`, `sync`) | Only for a TUI or network work. A plain CLI stays sync. |
| SQLite, sync | `rusqlite` (`bundled`) | The default for CLIs. Bundled, so no system SQLite needed. |
| SQLite, async | `sqlx` (`sqlite`, `runtime-tokio`, `macros`, `migrate`) | Only in a tokio app that wants compile-time checked queries and migrations. |
| Platform dirs | `directories` | When the tool needs OS-native dirs; otherwise the hand-rolled `RUST_TEMPLATE_HOME` in `src/domain/paths.rs`. |
| Shell completions | `clap_complete` | |
| Typo suggestions | `strsim` | "did you mean" for names. |
| Fuzzy picker | `nucleo` | Interactive fuzzy selection in a TUI. |
| Process timeout | `wait-timeout` | Timeouts on spawned processes, on Windows too. |
| Hashing | `sha2` | Content hashes, dedupe. |
| URL encoding | `percent-encoding` | |
| TUI text input | `tui-input` | |
| Clipboard | `arboard` | |
| Parallelism | `rayon` | Only once a measurement shows the need. |
| Iterator extras | `itertools` | Only when std iterators fall short. |
| Property tests (dev) | `proptest` | When a bug shows example tests missed a case. |

## Rejected

| Crate | Instead | Why |
| --- | --- | --- |
| `chrono` | `jiff` | jiff handles time zones and spans correctly with a smaller API surface. |
| `once_cell`, `lazy_static` | `std::sync::LazyLock` | In std since 1.80. |
| `color-eyre` | `anyhow` | Pretty reports add little in a CLI and nothing in a TUI. |
| `git2` | spawn `git` | libgit2 lags git and needs native builds. `gix` only if spawning becomes a measured problem. Banned in `deny.toml`. |
| `openssl` | rustls | No system OpenSSL to install or patch. `openssl-sys` banned in `deny.toml`. |
| `assert_cmd` | `std::process::Command` + `env!("CARGO_BIN_EXE_<name>")` | std covers it. |

## Adding a license

`deny.toml` allows permissive licenses only. When a chosen crate needs a license not on the
list, add it with a comment naming that crate. Never add one to unblock a crate that is not
chosen.

# Errors, logging, output

## Errors

- Everything below `main` returns `Result<_, AppError>`. `AppError` is one enum in
  `src/error.rs`, built with `thiserror`.
- `anyhow` only in `main.rs`, for context at the boundary (`.context("reading config")`).
- A variant's message says what went wrong and, when there is one, what to do about it:
  `"no home directory; set RUST_TEMPLATE_HOME"`.
- Wrap foreign errors with `#[from]` when the conversion is lossless, or a variant with fields
  when the caller needs to know which file or which command failed.
- Never swallow an error. `let _ =` on a `Result` needs a comment saying why ignoring it is
  right (for example, the receiver is gone because the app already quit).
- Per-item failures in a batch (one repo of many, one file of many) log a warning and
  continue. Failures that make the result wrong (the database, the config) are fatal.
- No panics in non-test code: the lints deny `unwrap`, `expect`, `panic`, indexing and
  unchecked arithmetic.

## Logging

- `tracing` macros everywhere; `tracing-subscriber` and `tracing-appender` set up once in
  `src/io/log.rs`.
- Logs go to `<RUST_TEMPLATE_HOME>/logs/rust-template.log`, never stdout (INV-1). stderr is
  for errors the user must see.
- Off unless `RUST_TEMPLATE_LOG` is set. The value is an `EnvFilter`:
  `RUST_TEMPLATE_LOG=debug`, or `info,rust_template::io=trace`.
- Log what a later debugging session needs: commands spawned, requests and statuses, files
  written, decisions taken. `warn!` on every failure that does not stop the run.

## Output

- stdout carries only what the command produces, so it can be piped.
- A `--json` output, when a tool has one, is stable: add fields, never rename or remove them.
- Errors print once, from `main`, through `anyhow`'s display.

# rust-template

One line on what the tool does.

## Commands

- `cargo nextest run --all-features`: tests (use this, not `cargo test`)
- `cargo clippy --all-targets --all-features`: must be clean; lints are `deny`, so this is the
  compile gate
- `cargo fmt --check`: formatting
- `cargo deny check`, `cargo machete`: dependency advisories, licenses, bans, unused crates
- `bacon clippy-all` / `bacon nextest`: watch mode
- `cargo run -- <args>`: run the tool; set `RUST_TEMPLATE_HOME` to a temp dir to keep your
  real config and logs clean. `RUST_TEMPLATE_LOG=debug` writes a log file under it.
<!-- tui -->
- `cargo run --features tui -- tui`: run the terminal UI
<!-- /tui -->
- `bd ready`: what is buildable next

The gate: fmt, clippy, nextest, all green before claiming anything works. The Stop hook in
`.claude/settings.json` runs it at the end of every turn and blocks while it is red. CI runs the
same gate plus deny and machete on Windows and Ubuntu.

## Workflow

1. Brainstorm the feature (superpowers), then a spec and plan under `docs/superpowers/`.
2. Track the work in beads (`bd`).
3. Build it with TDD, one behaviour at a time: red, green, refactor.
4. When a design settles, fold the decisions into the tracked docs: `README.md` for behaviour
   and the reasoning behind it, `docs/invariants.md` for rules with the tests that pin them,
   and `CLAUDE.md` for the module map.

Specs, plans and beads are local only. They are gitignored and never pushed (never
`bd dolt push`). Anything that must outlive the machine goes into the three tracked docs.

## Decided, do not ask

Everything in `docs/conventions/` and in the `Cargo.toml` comments is BK's standing preference:
crates, layout, architecture, testing, errors, style, release. Brainstorming, grilling and
planning sessions treat it as decided. Ask only about features, the domain, and real conflicts
between a feature and a convention; when you raise a conflict, name the convention.

## Hard rules

- Never weaken `[lints]` in `Cargo.toml` to make code compile. `#[allow(clippy::...)]` goes on
  one item only, with a one-line comment saying why. Ask BK before relaxing
  `arithmetic_side_effects` or `as_conversions`.
- No `unsafe` (`unsafe_code = "forbid"`).
- Pure core, thin IO shell: only `src/io/` and `src/main.rs` do IO.
- Never break an invariant in `docs/invariants.md`. An invariant without a test is a bug.
- Never accept a snapshot you have not read.
- No crate outside the `Cargo.toml` catalog without asking BK. Never an alternative for a job a
  listed crate does.
- No test needs network, a GPU, or the user's real config.

## Commits

Conventional Commits. release-plz builds `CHANGELOG.md` from the subjects, so a subject
describes the change for a user: never a bead id, never "this commit".
`feat: greet falls back to world for a blank name`, not `feat: rt-3 ...`.

## Map

```
src/
  main.rs           clap, logging init, dispatch; anyhow only here       [wiring]
  error.rs          AppError (thiserror)
  domain.rs         pure: facts in, plan out                             [pure]
    greet.rs        greeting text
    paths.rs        RUST_TEMPLATE_HOME / ~/.config/rust-template
  io.rs             everything that touches the outside world           [IO]
    log.rs          tracing to <home>/logs/rust-template.log
<!-- tui -->
    terminal.rs     TUI event loop; the only crossterm import           [IO]
  app/mod.rs        Message, Command, App::update                        [pure]
  ui/mod.rs         draw(&App, &mut Frame)                               [pure]
<!-- /tui -->
tests/
  cli.rs            the binary end to end, RUST_TEMPLATE_HOME in a temp dir
```

Module boundaries may shift; the pure/IO split does not.

## Conventions

Read the one that matches what you are about to do:

- `docs/conventions/architecture.md`: before adding a module, a trait, or anything with IO.
- `docs/conventions/testing.md`: before writing or changing a test.
- `docs/conventions/errors.md`: before adding an error variant, a log line, or output.
- `docs/conventions/style.md`: before writing code; the lint cheat sheet is there.
- `docs/conventions/crates.md`: before adding or uncommenting a dependency.
- `docs/releasing.md`: before touching versions, tags, or release config.

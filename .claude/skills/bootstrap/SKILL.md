---
name: bootstrap
description: Turn a fresh copy of rust-template into a named Rust CLI, TUI or port. Use once, right after creating a repo from the template, or when the user says "bootstrap", "set up this template", "new project from template". Renames placeholders, prunes the unused half, then hands off to brainstorming.
---

# Bootstrap

Run once in a repo created from rust-template. Every step is mechanical; the only questions are
in step 1. This file is excluded from every edit and every check below, and deleted in step 8.

Scope used throughout: "every tracked file" means `git ls-files`, minus
`.claude/skills/bootstrap/SKILL.md`. Cargo.lock and snapshot files are included.

## 1. Ask

Use AskUserQuestion, one batch:
- Name: kebab-case crate and binary name (also the repo name).
- Description: one line, for Cargo.toml, README, CLAUDE.md and `--help`.
- Kind: CLI, TUI, or port. For a port also ask where the original lives (path or URL) and
  whether the port is a CLI or a TUI.
- Targets: all 5 (default) or a subset.

Derive: `snake` = name with `-` replaced by `_`; `SCREAM` = `snake` uppercased.

## 2. Rename

In every tracked file, case-sensitive, in this order (for example
`git ls-files -z -- . ':!.claude/skills/bootstrap' | xargs -0 sed -i 's/RUST_TEMPLATE_/<SCREAM>_/g'`):
1. `RUST_TEMPLATE_` with `<SCREAM>_`
2. `rust_template` with `<snake>`
3. `rust-template` with `<name>` (this also covers the GitHub URLs)
4. `One line on what the tool does.` with the description

Set `keywords` in `Cargo.toml` to 1 to 5 words from the description.

Check: `git grep -in "rust.template\|one line on what" -- . ':!.claude/skills/bootstrap'`
prints nothing.

## 3. Prune

For the Cargo.toml edits, delete the named lines together with the blank line that follows
them. For the `.md` edits, skip this SKILL.md, and afterwards collapse any run of blank lines
to one and leave no blank line at the end of the file.

### CLI

- `Cargo.toml`: delete the `[features]` table and its two comment lines, and the
  ``# TUI, behind the `tui` feature.`` line with the three lines under it. Then add these three
  lines directly under `# Catalog. One line per job, with the rule for using it.`:
  ```toml
  # ratatui = "0.30"                               # TUI rendering; TestBackend for snapshots
  # crossterm = "0.29"                             # TUI terminal events; only io/terminal.rs
  # tokio = { version = "1", features = ["rt-multi-thread", "macros", "sync"] } # TUI or network only
  ```
- `git rm -rf src/app src/ui src/io/terminal.rs` (the rename left them modified, so `-f`).
- `src/main.rs`: delete the `#[cfg(feature = "tui")] mod app;` and `mod ui;` pairs, the
  `/// Open the terminal UI.` `#[cfg(feature = "tui")] Tui,` variant, and the
  `#[cfg(feature = "tui")] Command::Tui => ...` match arm.
- `src/io.rs`: delete `#[cfg(feature = "tui")]` and `pub mod terminal;`.
- `.github/workflows/ci.yml`: delete the `# Catches code that only compiles with the tui
  feature on.` comment and the `cargo clippy --all-targets` step under it.
- Every `.md` file: delete each `<!-- tui -->` ... `<!-- /tui -->` block, markers included.

### TUI

- `Cargo.toml`: delete the `[features]` table and its two comment lines. Replace the
  ``# TUI, behind the `tui` feature.`` line and the three lines under it with:
  ```toml
  # TUI.
  ratatui = "0.30"                                   # rendering; TestBackend for snapshots
  crossterm = "0.29"                                 # terminal events; only io/terminal.rs
  tokio = { version = "1", features = ["rt-multi-thread", "macros", "sync"] } # TUI or network only
  ```
- Delete every `#[cfg(feature = "tui")]` line, keeping the item under it.
- `src/main.rs`: the TUI becomes the no-command default.
  - `Cli.command` becomes `Option<Command>`.
  - Delete the `/// Open the terminal UI.` `Tui,` variant.
  - Match arms: `Some(Command::Greet { name }) => { ... }` and `None => io::terminal::run()?,`.
  - Under the `/// <description>` doc comment on `Cli`, add `///` and
    `/// Run without a command to open the terminal UI.`
- `.github/workflows/ci.yml`: delete the `# Catches code that only compiles with the tui
  feature on.` comment and the `cargo clippy --all-targets` step under it.
- `CLAUDE.md`: replace the line ``- `cargo run --features tui -- tui`: run the terminal UI``
  with ``- `cargo run`: open the terminal UI (no command)``.
- Every `.md` file: delete only the `<!-- tui -->` and `<!-- /tui -->` lines, keeping the
  content between them.

### Targets (both kinds)

Only if the user picked a subset:
- `dist-workspace.toml`: set `targets` to the subset. With no unix target left, set
  `installers = ["powershell"]`. With no Windows target left, set `installers = ["shell"]`.
- `README.md` `## Install`: delete the "macOS and Linux:" paragraph and its code block when
  the shell installer is gone, or the "Windows (PowerShell):" ones when powershell is gone.

## 4. Port

Only for a port. Read the original: its README, help output (`<tool> --help` and each
subcommand's help), and code. Write `docs/feature-inventory.md`:

```markdown
# Feature inventory

Every behaviour of <original>, the reference for the port. Check an item off when the port
does it and a test pins it. Delete this file once every item is checked.

- [ ] F1: <one behaviour, observable from outside>
```

Number items F1.. in the order a user meets them. Behaviours only, no implementation notes.
Add to `CLAUDE.md` under `## Workflow`: "Port of <original>. `docs/feature-inventory.md` is
the behaviour reference until parity."

## 5. Reset

- `README.md`: delete the `## Using this template` section, from its heading up to the next
  `##` heading.
- `bd init` (local only; never `bd dolt push`). Then `git status`: delete every file `bd init`
  created that this repo does not need, and keep only the beads lines `.gitignore` already has.
- If `dist` is installed, run `dist generate` (usually no diff: the build matrix is computed at
  release time). If it is not installed, tell the user to run it before the first release.

## 6. Verify and commit

TUI only, snapshots first: `cargo nextest run --all-features` fails on
`ui::tests::home_screen`, because the renamed title changed the border width. Run
`cargo insta pending-snapshots`, read the `.snap.new` next to the old snapshot, and accept with
`cargo insta accept` only when the one difference is the title line. Anything else is a defect:
stop and report it.

Then the gate, every command green:
```bash
cargo fmt --check
cargo clippy --all-targets --all-features
cargo nextest run --all-features
cargo deny check
cargo machete
git grep -in "rust.template\|one line on what\|feature = \"tui\"\|<!-- /\?tui" -- . ':!.claude/skills/bootstrap'
```
The last command must print nothing. Fix what the rename or prune broke. If a failure was
already there before bootstrap, stop and report it to the user; never commit a red gate.

```bash
git add -A
git commit -m "chore: bootstrap from rust-template"
```

## 7. Tell the user

Print these manual GitHub steps:
- Settings > Secrets and variables > Actions: add `RELEASE_PLZ_TOKEN`, a fine-grained PAT on
  this repo with Contents and Pull requests read/write. Without it releases silently do
  nothing.
- Settings > Actions > General: allow GitHub Actions to create and approve pull requests.

## 8. Hand off

Delete `.claude/skills/bootstrap/` and commit `chore: remove bootstrap skill`. Then invoke the
brainstorming skill for the first feature, opening with: "Conventions in `docs/conventions/`
and `Cargo.toml` are decided. Ask only about features, the domain, and real conflicts."

---
name: bootstrap
description: Turn a fresh copy of rust-template into a named Rust CLI, TUI or port. Use once, right after creating a repo from the template, or when the user says "bootstrap", "set up this template", "new project from template". Renames placeholders, prunes the unused half, then hands off to brainstorming.
---

# Bootstrap

Run once in a repo created from rust-template. Every step is mechanical; the only questions are
in step 1. Never edit this file during the run; it is deleted in step 8.

## 1. Ask

Use AskUserQuestion, one batch:
- Name: kebab-case crate and binary name (also the repo name).
- Description: one line, for Cargo.toml, README and CLAUDE.md.
- Kind: CLI, TUI, or port. For a port also ask where the original lives (path or URL) and
  whether the port is a CLI or a TUI.
- Targets: all 5 (default) or a subset.

Derive: `snake` = name with `-` replaced by `_`; `SCREAM` = `snake` uppercased.

## 2. Rename

In every tracked file (`git ls-files`) except `.claude/skills/bootstrap/SKILL.md`,
case-sensitive, in this order:
1. `https://github.com/bjornkpu/rust-template` with `https://github.com/bjornkpu/<name>`
2. `RUST_TEMPLATE_` with `<SCREAM>_`
3. `rust_template` with `<snake>`
4. `rust-template` with `<name>`
5. `One line on what the tool does.` with the description

Rename snapshot files whose names contain `rust_template` (`git mv`).
Check: `git grep -in "rust.template" -- . ':!.claude/skills/bootstrap'` prints nothing.

Screen snapshots now differ from the renamed code (the title width changed); step 6 reviews
them.

## 3. Prune

CLI:
- `Cargo.toml`: delete the `[features]` table (with its comment) and the
  `# TUI, behind the tui feature.` section. Add its three crates to the top of the commented
  catalog, commented out, without `optional = true`, each keeping its comment.
- Delete `src/app/`, `src/ui/` (with its `snapshots/`), `src/io/terminal.rs`.
- Delete every `#[cfg(feature = "tui")]` item: the `mod app;` and `mod ui;` lines and the
  `Tui` variant and match arm in `src/main.rs`, and `pub mod terminal;` in `src/io.rs`.
- In every `.md` file, delete each `<!-- tui -->` ... `<!-- /tui -->` block, markers included.

TUI:
- `Cargo.toml`: delete the `[features]` table (with its comment) and `, optional = true` from
  the three TUI lines. Rename the section comment to `# TUI.`
- Remove every `#[cfg(feature = "tui")]` attribute, keeping the items.
- Make the TUI the default: `command: Option<Command>` in `Cli`, `None` runs
  `io::terminal::run()?`, and remove the `Tui` variant.
- In every `.md` file, delete only the marker lines, keeping the content. In `CLAUDE.md`
  replace `cargo run --features tui -- tui` with `cargo run`.

Both: if the user picked a subset of targets, edit `targets` in `dist-workspace.toml`, and
drop the `shell` installer when no unix target is left.

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

- `version = "0.0.0"` in `Cargo.toml`.
- `CHANGELOG.md` back to its header only.
- `README.md`: delete the `## Using this template` section.
- `docs/invariants.md`: keep INV-1, it still holds.
- `bd init` (local only; never `bd dolt push`). Then `git status`: delete any file `bd init`
  generated that this repo does not need, and keep the `.gitignore` lines for beads.
- If `dist` is installed run `dist generate`, else tell the user to run it before the first
  release.

## 6. Verify and commit

Run the gate: `cargo fmt --check`, `cargo clippy --all-targets --all-features`,
`cargo nextest run --all-features`, `cargo deny check`, `cargo machete`. Fix what the rename or
prune broke. Review changed snapshots with `cargo insta pending-snapshots`, read each one, and
accept only when it shows exactly the rename.

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

# rust-template

One line on what the tool does.

## Using this template

This repo is a GitHub template for BK's Rust tools. It carries the conventions (crates,
lints, layout, tests, errors, release) and the Claude Code guardrails, so a new project starts
at the features.

1. On GitHub, click **Use this template** and create the new repo.
2. Clone it and open Claude Code in it.
3. Run `/bootstrap`. It asks for the name, description and kind (CLI, TUI, or port), renames
   everything, removes what the project does not need, and starts brainstorming the first
   feature.

This section is removed by `/bootstrap`.

## Install

macOS and Linux:

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/bjornkpu/rust-template/releases/latest/download/rust-template-installer.sh | sh
```

Windows (PowerShell):

```powershell
irm https://github.com/bjornkpu/rust-template/releases/latest/download/rust-template-installer.ps1 | iex
```

Update later with `rust-template-update`.

## Usage

```sh
rust-template greet BK
```

## Logging

Set `RUST_TEMPLATE_LOG` to a level (`debug`) or a filter (`info,rust_template::io=trace`).
Logs go to `~/.config/rust-template/logs/rust-template.log`, or under `RUST_TEMPLATE_HOME`
when that is set. Nothing is logged to the terminal.

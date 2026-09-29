# Architecture

## Pure core, thin IO shell

The shell gathers facts, a pure function decides, the shell executes. Facts in, plan out.

- `src/domain/` is pure. It takes and returns plain data: no filesystem, no network, no
  processes, no clock, no environment variables. Anything it needs from the world arrives as
  a parameter.
- `src/io/` does all IO: reading files, spawning processes, HTTP, the terminal, logging.
- `src/main.rs` is wiring: parse arguments, start logging, gather facts through `io`, call
  `domain`, execute the result through `io`. `anyhow` lives here and nowhere else.

Because `domain` is plain data, it is tested directly with no mocks and no fakes. When a
decision is hard to test, the IO has leaked into it: move the IO out and pass its result in.

A decision with several effects returns them as data (a `Vec<Step>`, a plan struct) for `io`
to execute. Tests assert on the plan; one integration test proves the executor runs it.

## Traits: the boundary rule

No traits with one implementation, with one exception: a boundary trait per external service
that is slow, costly, or non-deterministic (a network API, a GPU model, an LLM). It has the
real implementation and a fake, and the fake is the point: tests never touch the service.

```rust
pub trait Forecast {
    async fn today(&self, city: &str) -> Result<Weather, AppError>;
}

pub struct HttpForecast { client: reqwest::Client, base: String }
impl Forecast for HttpForecast { /* real call */ }

#[cfg(test)]
pub struct FakeForecast(pub Weather);
#[cfg(test)]
impl Forecast for FakeForecast {
    async fn today(&self, _city: &str) -> Result<Weather, AppError> { Ok(self.0.clone()) }
}
```

Local things are not boundaries. git, SQLite and the filesystem are used for real in tests,
inside a temp dir (see `testing.md`).

## Typestate

When a value moves through states that must never be mixed at runtime (unvalidated then
validated, draft then sent), make each state its own type and make the transition a function
that consumes one and returns the next. The compiler then rejects the mix-up.

## Paths

`RUST_TEMPLATE_HOME` overrides everything; without it the tool uses
`~/.config/rust-template`. Config, logs and data live under that one dir until a tool needs
the XDG split.

<!-- tui -->
## Terminal UI (Elm style)

Three pure pieces and one IO shell:

- `Message` (`src/app/mod.rs`): our own enum for everything that can happen (`Key(..)`,
  `Tick`, `DataLoaded(..)`). crossterm events become `Message`s in `src/io/terminal.rs`, the
  only file that imports crossterm.
- `App::update(&mut self, Message) -> Vec<Command>`: the only place state changes. No IO.
  Side effects come back as `Command`s.
- `ui::draw(&App, &mut Frame)` (`src/ui/mod.rs`): pure render, never mutates.
- `src/io/terminal.rs`: owns the terminal, runs the draw/update loop, executes `Command`s.
  Background work (fetches, long jobs) runs in tokio tasks that never touch `App`; they send
  `Message`s down an mpsc channel. `Command::Quit` ends the loop; nothing calls `exit`, so the
  terminal is always restored.

Use typestate for screens whose transitions must not be mixed.
<!-- /tui -->

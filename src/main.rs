use anyhow::{Context, Result};
pub mod utils;

/// This is a bare minimum example. There are many approaches to running an application loop, so
/// this is not meant to be prescriptive. It is only meant to demonstrate the basic setup and
/// teardown of a terminal application.
///
/// A more robust application would probably want to handle errors and ensure that the terminal is
/// restored to a sane state before exiting. This example does not do that. It also does not handle
/// events or update the application state. It just draws a greeting and exits when the user
/// presses 'q'.
// #[tokio::main]
fn main() -> Result<()> {
    let mut terminal = ratatui_project::setup_terminal().context("setup failed")?;
    ratatui_project::run(&mut terminal).context("setup failed")?;
    ratatui_project::restore_terminal(&mut terminal).context("restore terminal failed")?;
    Ok(())
}
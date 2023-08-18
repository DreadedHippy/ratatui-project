use std::io::{self, Stdout};
mod events;
mod interfaces;
mod utils;
pub mod banking_system;

use events::input_event;
use interfaces:: app::{AppInterface, InputMode};
use anyhow::{Context, Result};
use crossterm::{
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{prelude::*, widgets::ScrollbarState};
use utils::{TABS, CustomInput};

/// Setup the terminal. This is where you would enable raw mode, enter the alternate screen, and
/// hide the cursor. This example does not handle errors. A more robust application would probably
/// want to handle errors and ensure that the terminal is restored to a sane state before exiting.
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
	let mut stdout = io::stdout();
	enable_raw_mode().context("failed to enable raw mode")?;
	execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
	Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

/// Restore the terminal. This is where you disable raw mode, leave the alternate screen, and show
/// the cursor.
pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
	disable_raw_mode().context("failed to disable raw mode")?;
	execute!(terminal.backend_mut(), LeaveAlternateScreen).context("unable to switch to main screen")?;
	terminal.show_cursor().context("unable to show cursor")
}

/// Run the application loop. This is where you would handle events and update the application
/// state. This example exits when the user presses 'q'. Other styles of application loops are
/// possible, for example, you could have multiple application states and switch between them based
/// on events, or you could have a single application state and update it based on events.
pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {	
	let mut app: AppInterface = AppInterface {
		tab_display: (TABS.iter().map(|x| x.to_string().to_owned()).collect(), 0),
		input: String::from(""),
		input_mode: InputMode::Normal,
		display_text: String::from("Welcome to Rust Bank, press a number to toggle between options"),
		hint_text: String::from("Press 'Enter' to select an option | Press 'q' to exit"),
		is_submitted: None,
		activity: None,
		should_quit: false,
		terminal: None,
		scrollbar_state: ScrollbarState::default(),
		scroll: 0
	};

	loop {
		app.draw(terminal);
		match app.input_mode {
			InputMode::Normal => {
				if let Some(v) = input_event() {
					match v {
						CustomInput::Number(n) => {
							app.select_tab(n-1);
							match n {
								1 => {
									app.set_display_text("Would you like to register?".to_string()).scroll_to_top()
								},
								2 => {
									app.set_display_text("Would you like to open an account?".to_string()).scroll_to_top()
								},
								3 => {
									app.set_display_text("Would you like to deposit money?".to_string()).scroll_to_top()
								},
								4 => {
									app.set_display_text("Would you like to withdraw some money?".to_string()).scroll_to_top()
								},
								5 => {
									app.set_display_text("Would you like to check your balance?".to_string()).scroll_to_top()
								},
								6 => {
									app.set_display_text("<!ADMIN ONLY> Would you like to see all users?".to_string()).scroll_to_top()
								},
								7 => {
									app.set_display_text("Would you like to close an account?".to_string()).scroll_to_top()
								},
								8 => {
									app.set_display_text("Would you like to update an account?".to_string()).scroll_to_top()
								},
								_ => {}
							}
						},

						CustomInput::Char(c) => {
							match c {
								'q' => break,
								b => {
									// let display = 
									app.display_text = format!("User pressed {}", b);
								}
							}		
						}

						CustomInput::Enter => {
							app.input_mode = InputMode::Editing;
							app.display_text = "You have entered edit mode. Press any button to begin typing".to_string();
							app.set_hint_mode(2)
							// app.start()
						},

						CustomInput::Up => {
							app.scroll('u');
						},
						CustomInput::Down => {
							app.scroll('d');
						}

						_ => {}
						
					}
				}
			},

			InputMode::Editing => {
				app.start(terminal);
			}
		}
	}
	Ok(())
}

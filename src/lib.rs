use std::io::{self, Stdout};
mod events;
mod interfaces;
mod utils;

use events::input_event;
use interfaces::{Interfaces, app::{AppInterface, InputMode}};

use anyhow::{Context, Result};

use crossterm::{
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{prelude::*, widgets::*};
use utils::{TABS, CustomInput};



// const OPTIONS: [&str; 8] = [
// 	"Tab1",
// 	"Tab2",
// 	"Tab3",
// 	"Tab4",
// 	"Tab5",
// 	"Tab6",
// 	"Tab7",
// 	"Tab8",
// ];

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
		display_text: vec![String::from("Welcome to Rust Bank, press a number to toggle between options")],
		hint_text: String::from("Press 'e' to enter Edit mode | Press 'q' to exit")
	};

	loop {
		terminal.draw(|f| {
			interfaces::ui(f, &app);
		})?;

		match app.input_mode {
			InputMode::Normal => {
				if let Some(v) = input_event() {
					if let CustomInput::Number(n) = v {
						app.select_tab(n-1);
						match n {
							1 => {
								app.display_text = vec!["Would you like to register?".to_string()]
							},
							2 => {
								app.display_text = vec!["Would you like to deposit money?".to_string()]
							},
							3 => {
								app.display_text = vec!["Would you like to withdraw some money?".to_string()]
							},
							4 => {
								app.display_text = vec!["Would you like to check your balance?".to_string()]
							},
							5 => {
								app.display_text = vec!["<!ADMIN ONLY> Would you like to see all users?".to_string()]
							},
							6 => {
								app.display_text = vec!["Would you like to close an account?".to_string()]
							},
							7 => {
								app.display_text = vec!["Would you like to update an account?".to_string()]
							},
							8 => {						
								app.display_text = vec!["Would you like to close exit?".to_string()]
							},
							_ => {}
						}
					}
		
					if let CustomInput::Char(c) = v {
						match c {
							'q' => break,
							'e' => {
								app.input_mode = InputMode::Editing;
								app.display_text = vec!["You have entered edit mode. Press any button to begin typing".to_string()];
								app.hint_text = String::from("Press 'Esc' to exit edit mode.");
							}
							b => {
								let display = format!("User pressed {}", b);
								app.display_text = vec![display];
							}
						}
		
						}
				}
			},

			InputMode::Editing => {
				terminal.show_cursor()?;
				if let Some(i) = input_event() {
					match i {
						CustomInput::Escape => {
							app.display_text = vec!["You have entered Normal mode".to_string()];
							app.hint_text = String::from("Press 'e' to enter Edit mode | Press 'q' to exit");
							app.input_mode = InputMode::Normal;
							app.input = "".to_string();
						},

						CustomInput::Backspace => {
							if app.input.len() > 0 {
								let l = app.input.len() - 1;
								app.input = app.input.chars().into_iter().take(l).collect();
							}
						},

						CustomInput::Left => {
							let (x_pos, y_pos) = terminal.get_cursor()?;
							if x_pos > 0 {
								terminal.set_cursor(x_pos - 1, y_pos);
							}
						}
						
						CustomInput::Right => {
							let (x_pos, y_pos) = terminal.get_cursor()?;
							if x_pos < app.input.len() as u16 + 2 {
								terminal.set_cursor(x_pos + 1, y_pos)?;
							}
						}
						_ => {
							let cursor_pos = terminal.get_cursor()?.0;
							let y = terminal.get_cursor()?.1;
							let after: String = app.input.chars().skip(cursor_pos as usize - 2).collect();
							app.input = app.input.chars().take(cursor_pos as usize - 2).collect::<String>() + &i.inner_value() + &after;
							terminal.set_cursor(cursor_pos + 1, y)?;
						}
					}
				}
			}
		}

	}
	Ok(())
}

// Render the application. This is where you would draw the application UI. This example just
// draws a greeting.

// pub fn render_mouse_event_paragraph(frame: &mut ratatui::Frame<CrosstermBackend<Stdout>>, paragraph: Paragraph) {
// 	frame.render_widget(paragraph, frame.size());
// }
// pub fn render_app(frame: &mut ratatui::Frame<CrosstermBackend<Stdout>>) {
// 	let greeting = Paragraph::new("Welcome to the Rust Bank! (press 'q' to quit)");
// 	frame.render_widget(greeting, frame.size());
// }

// pub fn render_diff_paragraph(frame: &mut ratatui::Frame<CrosstermBackend<Stdout>>) {
// 	frame.render_widget(Interfaces::paragaph(), frame.size());
// }

// pub fn render_another_paragraph(frame: &mut ratatui::Frame<CrosstermBackend<Stdout>>) {
// 	frame.render_widget(Interfaces::another_paragaph(), frame.size());
// }

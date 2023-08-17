use std::time::Duration;
use anyhow::{Context, Result};
pub mod user_input;

macro_rules! ok_or_return_none {
	($contents:expr) => {
		match $contents {
			Result::Ok(val) => val,
			Result::Err(_) => return None,
		}
	};
}
use crossterm::event::{self, Event, KeyCode};

use crate::utils::CustomInput;
/// Check if the user has pressed 'q'. This is where you would handle events. This example just
/// checks if the user has pressed 'q' and returns true if they have. It does not handle any other
/// events. There is a 250ms timeout on the event poll so that the application can exit in a timely
/// manner, and to ensure that the terminal is rendered at least once every 250ms.
// pub fn should_quit() -> Result<bool> {
// 	if event::poll(Duration::from_millis(250)).context("event poll failed")? {
// 		if let Event::Key(key) = event::read().context("event read failed")? {
// 			return Ok(KeyCode::Char('q') == key.code);
// 		}
// 	}
// 	Ok(false)
// }

// pub fn new_parargraph() -> Result<bool> {
// 	if event::poll(Duration::from_millis(250)).context("event poll failed")? {
// 		if let Event::Key(key) = event::read().context("event read failed")? {
// 			return Ok(KeyCode::Char('n') == key.code);
// 		}
// 	}
// 	Ok(false)
// }

// pub fn another_paragraph() -> Result<bool> {
// 	if event::poll(Duration::from_millis(250)).context("event poll failed")? {
// 		if let Event::Key(key) = event::read().context("event read failed")? {
// 			return Ok(KeyCode::Char('a') == key.code);
// 		}
// 	}
// 	Ok(false)
// }

pub fn input_event() -> Option<CustomInput> {
	if ok_or_return_none!(event::poll(Duration::from_millis(250)).context("event poll failed")) {
		if let Event::Key(key_event) = ok_or_return_none!(event::read().context("...")){
			if let event::KeyEventKind::Press = key_event.kind {
				match key_event.code {
					KeyCode::Char(c) => {
						if c >= '1' && c <= '8' {
							return Some(CustomInput::Number(c as usize - '0' as usize))
						} else {
							return Some(CustomInput::Char(c))
						}
					}
	
					KeyCode::Esc => {
						return Some(CustomInput::Escape)
					},
	
					KeyCode::Backspace => {
						return Some(CustomInput::Backspace)
					},

					KeyCode::Enter => {
						return Some(CustomInput::Enter)
					}

					KeyCode::Left => {
						return Some(CustomInput::Left)
					},
					
					KeyCode::Right => {
						return Some(CustomInput::Right)
					}
					_ => { return None}
				}
			}
		}
	}
	None
	// anyhow!("something");
}

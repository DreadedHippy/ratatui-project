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
pub fn input_event() -> Option<CustomInput> {
	if ok_or_return_none!(event::poll(Duration::from_millis(250)).context("event poll failed")) {
		if let Event::Key(key_event) = ok_or_return_none!(event::read().context("...")){
			if let event::KeyEventKind::Press = key_event.kind {
				match key_event.code {
					KeyCode::Char(c) => {
						if c >= '0' && c <= '9' {
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

					KeyCode::Up => {
						return Some(CustomInput::Up)
					},
					
					KeyCode::Down => {
						return Some(CustomInput::Down)
					}
					_ => { return None}
				}
			}
		}
	}
	None
	// anyhow!("something");
}

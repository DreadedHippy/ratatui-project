// use std::default;
use std::{io::{self, Stdout}, time::Duration};
use ratatui::prelude::*;
use crate::{banking_system::start_bank, utils::CustomInput, events::input_event};

use super::Interfaces;

const BROWSERS: &'static [&'static str] = &["firefox", "chrome"];

#[derive(Clone)]
pub enum InputMode {
	Normal,
	Editing
}
type AppTerminal = Terminal<CrosstermBackend<Stdout>>;
// #[derive(Clone)]
pub struct AppInterface<'a> {
	pub tab_display: (Vec<String>, usize),
	pub input: String,
	pub input_mode: InputMode,
	pub display_text: String,
	pub hint_text: String,
	pub is_submitted: Option<bool>,
	pub activity: Option<u8>,
	pub should_quit: bool,
	pub terminal: Option<&'a mut AppTerminal>
}

impl AppInterface<'_> {
	pub fn select_tab(&mut self, tab_index: usize) {
		self.tab_display.1 = tab_index
	}

	pub fn start(&mut self, terminal:&mut AppTerminal) {
		let selection = self.tab_display.1 + 1;
		match selection {
			1 => {
				self.display_text = "Creating new customer".to_string();
				self.create_new_customer(terminal);
			},
			2 => {},
			3 => {},
			4 => {},
			5 => {},
			6 => {},
			7 => {},
			8 => {},
			_ => {},
		}
		// start_bank(selection as u8)
	}

	fn create_new_customer(&mut self, terminal:&mut AppTerminal) {
		self.activity = Some(1);
		self.set_display_text(String::from("Creating new customer.")).draw(terminal);
		std::thread::sleep(Duration::from_millis(500));
		self.set_display_text(String::from("Creating new customer..")).draw(terminal);
		std::thread::sleep(Duration::from_millis(500));
		self.set_display_text(String::from("Creating new customer...")).draw(terminal);
		std::thread::sleep(Duration::from_millis(1000));
		self.set_display_text(String::from("Enter your name")).draw(terminal);
		let name = self.get_input(terminal);
		self.display_text = format!("The name entered is {}", name);
		// let pin = self.get_user_input();
		// self.display_text = format!("The name entered is {}, and the pin is {}", name.unwrap_or(default.clone()), pin.unwrap_or(default));
		
	}

	pub fn get_input(&mut self, terminal:&mut AppTerminal) -> String {
		loop {
			self.draw(terminal);
			if let Some(i) = input_event() {
				match i {
					CustomInput::Char(v) => { self.input.push_str(&v.to_string())},
					CustomInput::Enter => {
						self.input_mode = InputMode::Normal;
						self.set_hint_mode(1);
						return self.input.clone();
					},
					_ => {}
				}
			}
		};
	}

	pub fn draw(&self, terminal:&mut AppTerminal) {
		terminal.draw(|f| {
			super::ui(f, &self)
		}).expect("Failed to draw interface");
	}

	pub fn set_hint_mode(&mut self, mode_id: u16) {
		// 1 is normal, 2 is input
		match mode_id {
			1 => {self.hint_text = String::from("Press 'Enter' to select an option | Press 'q' to exit")},
			2 => {self.hint_text = String::from("Press 'Esc' to exit edit mode.")},
			_ => {}
		}
	}

	pub fn set_display_text(&mut self, display_text: String) -> &mut Self{
		self.display_text = display_text;
		return self;
	}
}
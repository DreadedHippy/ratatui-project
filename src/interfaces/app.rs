// use std::default;
use std::{io::Stdout, time::Duration, thread::sleep, fmt::format};
use anyhow::{Result, Ok, bail};
use ratatui::prelude::*;
use crate::{banking_system::{self, handlers::Events as bank_sys}, utils::CustomInput, events::input_event};

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
				if !self.create_new_customer(terminal).is_ok() {
					self.display_text = "User cancelled activity".to_string();
					self.input_mode = InputMode::Normal;
					self.set_hint_mode(1);
					return
				}
			},
			2 => {
				self.set_display_text("Depositing into account".to_string()).draw(terminal);
				match self.deposit_money(terminal) {
					Err(i) => {
						self.set_display_text(i.to_string()).draw(terminal);
					},
					_ => {},
				}
				self.input_mode = InputMode::Normal;
				self.set_hint_mode(1);
				return
			},
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

	fn create_new_customer(&mut self, terminal:&mut AppTerminal) -> Result<()> {
		self.set_display_text(String::from("Creating new customer...")).draw(terminal).pause(500);
		self.set_display_text(String::from("Enter your name")).draw(terminal);

		let name = self.get_input(terminal, 'c')?;
		self.set_display_text(String::from("Enter your pin code (4 digits)")).draw(terminal);
		let pin_code = self.get_input(terminal, 'n')?;
		match bank_sys::new_customer(name, pin_code) {
			std::result::Result::Ok(i) => {
				self.set_display_text(format!("{}\nCustomer: {:#?}", i.0, i.1)).draw(terminal).pause(1500);
			},
			Err(e) => {
				self.set_display_text(e).draw(terminal).pause(1000);
				self.set_display_text("Try again".to_string()).draw(terminal).pause(1000);
				return self.create_new_customer(terminal);
			}
		}
		// self.display_text = banking_system::handlers::Events::new_customer(name, pin_code).unwrap_or("Failed to save user".to_string());
		Ok(())
	}

	fn deposit_money(&mut self, terminal: &mut AppTerminal) -> anyhow::Result<(), anyhow::Error> {
		self.set_display_text("Enter your name".to_string()).draw(terminal); //prompt the user for name
		let name = self.get_input(terminal, 'c')?; // get the name
		self.set_display_text("Enter your PIN code".to_string()).draw(terminal); //prompt the user for PIN
		let pin_code = self.get_input(terminal, 'a')?; // get the pin code

		let (customer_index, customer) = match banking_system::handlers::get_customer(name, pin_code) { // find the customer in db
			Some(i) => i, // else, assign the values to "customer_index" and "customer"
			None => bail!("Could not find your profile, try registering"), // if unsuccessful, throw error
		};

		let p =  "Select the account number of the account you would like to deposit into";
		self.set_display_text(format!("{:?}\n {p}", customer.accounts())).draw(terminal); // Prompt the user for account number

		let account_number = self.get_input(terminal, 'n')?; // get account number
		if !customer.accounts.iter().any(|acc| acc.account_number == account_number) { // search for account number among user accounts
			bail!("The selected account is invalid") // if not found, throw error
		}

		self.set_display_text("Enter the amount you would like to deposit (limit of 99999): ".to_string()); // ask for input amount
		let amount = self.get_input(terminal, 'a')?.parse::<u32>()?; // get the input amount

		match bank_sys::deposit_money(customer_index, customer, account_number, amount) {
			std::result::Result::Ok(i) => {
				self.set_display_text(format!("Deposit successful, your new balance is {}", i));
			},
			Err(e) => {
				bail!("Failed to deposit amount, {e}")
			}
		}
		Ok(())

	}

	fn get_input(&mut self, terminal:&mut AppTerminal, input_type: char) -> Result<String> {
		self.set_hint_mode(2);
		loop {
			self.draw(terminal);
			if let Some(input_event) = input_event() {
				match (input_event, input_type) {
					(CustomInput::Char(c), 'c') =>{
						self.input.push_str(&c.to_string())
					},
					(CustomInput::Number(n), t) => {
						match t {
							'n' => 	{
								if self.input.len() < 4 {
									self.input.push_str(&n.to_string())
								}
							},
							'a' => 	{
								if self.input.len() < 5 {
									self.input.push_str(&n.to_string())
								}
							},
							_ => 	{},
						}
					},
					(CustomInput::Backspace, _) => {
						if self.input.is_empty() { continue };
						self.input.truncate(self.input.len() - 1);
					},
					(CustomInput::Escape, _) => {
						self.input = String::new();
						bail!("User cancelled activity");
					}
					(CustomInput::Enter, t) => {
						if self.input.is_empty() {
							continue
						}
						if t == 'n' && self.input.len() < 4 {
							continue
						}
						self.input_mode = InputMode::Normal;
						self.set_hint_mode(1);
						let j = self.input.clone();
						self.input = String::new();
						return Ok(j);
					},
					_ => {}
				}
			}
		};
	}

	pub fn draw(&mut self, terminal:&mut AppTerminal) -> &mut Self{
		terminal.draw(|f| {
			super::ui(f, &self)
		}).expect("Failed to draw interface");
		self
	}

	fn pause(&mut self, duration: u64) -> &mut Self {
		sleep(Duration::from_millis(duration));
		self
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
		self
	}
}
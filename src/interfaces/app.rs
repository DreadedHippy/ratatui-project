use std::{io::Stdout, time::Duration, thread::sleep};
use anyhow::{Result, Ok, bail};
use ratatui::{prelude::*, widgets::ScrollbarState};
use crate::{banking_system::{self, handlers::Events as bank_sys}, utils::CustomInput, events::input_event};

#[derive(Clone, Default)]
pub enum InputMode {
	#[default]
	Normal,
	Editing
}
type AppTerminal = Terminal<CrosstermBackend<Stdout>>;
#[derive(Default)]
pub struct AppInterface<'a> {
	pub tab_display: (Vec<String>, usize),
	pub input: String,
	pub input_mode: InputMode,
	pub display_text: String,
	pub hint_text: String,
	pub is_submitted: Option<bool>,
	pub activity: Option<u8>,
	pub should_quit: bool,
	pub terminal: Option<&'a mut AppTerminal>,
	pub scrollbar_state: ScrollbarState,
	pub scroll: usize
}

impl AppInterface<'_> {
	pub fn select_tab(&mut self, tab_index: usize) {
		if tab_index > 7 {
			return
		}
		self.tab_display.1 = tab_index
	}

	pub fn start(&mut self, terminal:&mut AppTerminal) {
		let selection = self.tab_display.1 + 1;
		match selection {
			1 => {
				self.set_display_text("Initializing operation...".to_string()).draw(terminal).pause(300);
				if !self.create_new_customer(terminal).is_ok() {
					self.display_text = "User cancelled activity".to_string();
					self.input_mode = InputMode::Normal;
					self.set_hint_mode(1);
					return
				}
			},
			2 => {
				self.set_display_text("Initializing operation...".to_string()).draw(terminal).pause(300);
				match self.create_bank_account(terminal) {
					Err(i) => {
						self.set_display_text(i.to_string()).draw(terminal);
					},
					_ => {}
				}
				self.input_mode = InputMode::Normal;
				self.set_hint_mode(1);
				return
			},
			3 => {
				self.set_display_text("Initializing operation...".to_string()).draw(terminal).pause(300);
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
			4 => {
				self.set_display_text("Initializing operation...".to_string()).draw(terminal).pause(300);
				match self.withdraw_money(terminal) {
					Err(i) => {
						self.set_display_text(i.to_string()).draw(terminal);
					},
					_ => {},
				}
				self.input_mode = InputMode::Normal;
				self.set_hint_mode(1);
				return
			},
			5 => {
				self.set_display_text("Initializing operation...".to_string()).draw(terminal).pause(300);
				match self.check_balances(terminal) {
					Err(i) => {
						self.set_display_text(i.to_string()).draw(terminal);
					},
					_ => {},
				}
				self.input_mode = InputMode::Normal;
				self.set_hint_mode(1);
				return
			},
			6 => {
				self.set_display_text("Initializing operation...".to_string()).draw(terminal).pause(300);
				match self.get_all_users(terminal) {
					Err(i) => {
						self.set_display_text(i.to_string()).draw(terminal);
					},
					_ => {},
				}
				self.input_mode = InputMode::Normal;
				self.set_hint_mode(1);
				return
			},
			7 => {
				self.set_display_text("Initializing operation...".to_string()).draw(terminal).pause(300);
				match self.close_bank_account(terminal) {
					Err(i) => {
						self.set_display_text(i.to_string()).draw(terminal);
					},
					_ => {},
				}
				self.input_mode = InputMode::Normal;
				self.set_hint_mode(1);
				return
			},
			8 => {
				self.set_display_text("Initializing operation...".to_string()).draw(terminal).pause(300);
				match self.update_bank_account(terminal) {
					Err(i) => {
						self.set_display_text(i.to_string()).draw(terminal);
					},
					_ => {},
				}
				self.input_mode = InputMode::Normal;
				self.set_hint_mode(1);
				return
			},
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
				self.set_display_text(format!("{}\n\nCustomer: {:#?}", i.0, i.1)).draw(terminal).pause(1500);
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
	
	fn create_bank_account(&mut self, terminal: &mut AppTerminal) -> anyhow::Result<(), anyhow::Error> {
		self.set_display_text("Enter your name: ".to_string()).draw(terminal); //prompt the user for name
		let name = self.get_input(terminal, 'c')?; // get the name
		self.set_display_text("Enter your PIN code: ".to_string()).draw(terminal); //prompt the user for PIN
		let pin_code = self.get_input(terminal, 'a')?; // get the pin code

		let (customer_index, customer) = match banking_system::handlers::get_customer(name, pin_code) { // find the customer in db
			Some(i) => i, // else, assign the values to "customer_index" and "customer"
			None => bail!("Could not find your profile, try registering"), // if unsuccessful, throw error
		};

		self.set_display_text("Enter the account type of the new account (c/s)".to_string()).draw(terminal);
		let account_type = self.get_input(terminal, 't')?; // get account type		

		self.set_display_text("Enter the account number of the new account".to_string()).draw(terminal);
		let account_number = self.get_input(terminal, 'n')?; // get account number

		if customer.accounts.iter().any(|acc| acc.account_number == account_number) { // search for account number among user accounts
			bail!("An account with the provided account number already exists") // if already exists, throw error
		}

		let account_info = (account_number, account_type);

		let result = match bank_sys::create_bank_account(customer_index, account_info) {
			Some(i) => i,
			None => bail!("Unable to create bank account")
		};
		self.set_display_text(format!("Bank account created successfully \n\n Account: {:#?}", result)).draw(terminal);
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
		self.set_display_text(format!("{p}: \n\n{:#?}", customer.accounts())).draw(terminal); // Prompt the user for account number

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

	
	fn withdraw_money(&mut self, terminal: &mut AppTerminal) -> anyhow::Result<(), anyhow::Error> {
		self.set_display_text("Enter your name".to_string()).draw(terminal); //prompt the user for name
		let name = self.get_input(terminal, 'c')?; // get the name
		self.set_display_text("Enter your PIN code".to_string()).draw(terminal); //prompt the user for PIN
		let pin_code = self.get_input(terminal, 'a')?; // get the pin code

		let (customer_index, customer) = match banking_system::handlers::get_customer(name, pin_code) { // find the customer in db
			Some(i) => i, // else, assign the values to "customer_index" and "customer"
			None => bail!("Could not find your profile, try registering"), // if unsuccessful, throw error
		};

		let p =  "Enter the account number of the account you would like to withdraw from";
		self.set_display_text(format!("{p}: \n\n{:#?}", customer.accounts())).draw(terminal); // Prompt the user for account number

		let account_number = self.get_input(terminal, 'n')?; // get account number
		if !customer.accounts.iter().any(|acc| acc.account_number == account_number) { // search for account number among user accounts
			bail!("The selected account is invalid") // if not found, throw error
		}

		self.set_display_text("Enter the amount you would like to withdraw (limit of 99999): ".to_string()); // ask for input amount
		let amount = self.get_input(terminal, 'a')?.parse::<u32>()?; // get the input amount

		match bank_sys::withdraw_money(customer_index, customer, account_number, amount) {
			std::result::Result::Ok(i) => {
				self.set_display_text(format!("Deposit successful, your new balance is {}", i));
			},
			Err(e) => {
				bail!("Failed to deposit amount, {e}")
			}
		}
		Ok(())
	}

	fn check_balances(&mut self, terminal: &mut AppTerminal) -> anyhow::Result<(), anyhow::Error> {
		self.set_display_text("Enter your name".to_string()).draw(terminal); //prompt the user for name
		let name = self.get_input(terminal, 'c')?; // get the name
		self.set_display_text("Enter your PIN code".to_string()).draw(terminal); //prompt the user for PIN
		let pin_code = self.get_input(terminal, 'a')?; // get the pin code

		let (_, customer) = match banking_system::handlers::get_customer(name, pin_code) { // find the customer in db
			Some(i) => i, // else, assign the values to "customer_index" and "customer"
			None => bail!("Could not find your profile, try registering"), // if unsuccessful, throw error
		};
		self.set_display_text(format!("Here are your accounts: \n\n{:#?}", customer.accounts())).draw(terminal); // Prompt the user for account number
		Ok(())
	}

	
	fn get_all_users(&mut self, terminal: &mut AppTerminal) -> anyhow::Result<(), anyhow::Error> {
		self.set_display_text("ENTER ADMIN CREDENTIALS".to_string()).draw(terminal); //prompt the user for admin credentials
		let admin_credentials = self.get_input(terminal, 'g')?; // get the admin credentials

		let users = match bank_sys::get_admin_info(admin_credentials) { // get the users using credentials
			std::result::Result::Ok(i) => i, // return credentials on success
			Err(e) => bail!(e) //throw error on fail
		};
		self.set_display_text(format!("USER LIST: \n\n{:#?}", users)).draw(terminal); // Display users

		Ok(())
	}

	
	fn close_bank_account(&mut self, terminal: &mut AppTerminal) -> anyhow::Result<(), anyhow::Error> {
		self.set_display_text("Enter your name".to_string()).draw(terminal); //prompt the user for name
		let name = self.get_input(terminal, 'c')?; // get the name
		self.set_display_text("Enter your PIN code".to_string()).draw(terminal); //prompt the user for PIN
		let pin_code = self.get_input(terminal, 'a')?; // get the pin code

		let (customer_index, customer) = match banking_system::handlers::get_customer(name, pin_code) { // find the customer in db
			Some(i) => i, // else, assign the values to "customer_index" and "customer"
			None => bail!("Could not find your profile, try registering"), // if unsuccessful, throw error
		};

		let p =  "Enter the account number of the account you would like to close";
		self.set_display_text(format!("{p}\n\n{:#?}", customer.accounts())).draw(terminal); // Prompt the user for account number

		let account_number = self.get_input(terminal, 'n')?; // get account number
		if !customer.accounts.iter().any(|acc| acc.account_number == account_number) { // search for account number among user accounts
			bail!("The selected account is invalid") // if not found, throw error
		}

		let updated_customer = match bank_sys::close_bank_account(customer, customer_index, account_number) { // close account and retrieve updated info
			std::result::Result::Ok(t) => t, // successful? assign value to "updated_customer"
			Err(e) => bail!(e) // fail? throw error
		};

		// format info into a string
		let p = format!("Your account has been closed successfully. New info: \n\n{:#?}", updated_customer);
		self.set_display_text(p).draw(terminal); // Display updated user
		Ok(())
	}

	
	fn update_bank_account(&mut self, terminal: &mut AppTerminal) -> anyhow::Result<(), anyhow::Error> {
		self.set_display_text("Enter your name".to_string()).draw(terminal); //prompt the user for name
		let name = self.get_input(terminal, 'c')?; // get the name
		self.set_display_text("Enter your PIN code".to_string()).draw(terminal); //prompt the user for PIN
		let pin_code = self.get_input(terminal, 'a')?; // get the pin code

		let (customer_index, customer) = match banking_system::handlers::get_customer(name, pin_code) { // find the customer in db
			Some(i) => i, // else, assign the values to "customer_index" and "customer"
			None => bail!("Could not find your profile, try registering"), // if unsuccessful, throw error
		};

		let p =  "Enter the account number of the account you would like to update";
		self.set_display_text(format!("{p}\n\n {:#?}", customer.accounts())).draw(terminal); // Prompt the user for old account number

		let old_account_number = self.get_input(terminal, 'n')?; // get account number
		if !customer.accounts.iter().any(|acc| acc.account_number == old_account_number) { // search for account number among user accounts
			bail!("The selected account is invalid") // if not found, throw error
		}

		let p =  "Enter the new account number you would like";
		self.set_display_text(format!("{p}")).draw(terminal); // Prompt the user for new account number

		let new_account_number = self.get_input(terminal, 'n')?; // get account number
		if customer.accounts.iter().any(|acc| acc.account_number == new_account_number) { // search for account number among user accounts
			bail!("The selected account number has already been assigned") // if already exists, throw error
		}

		let updated_account = match bank_sys::update_bank_account(customer, customer_index, old_account_number, new_account_number) { // close account and retrieve updated info
			std::result::Result::Ok(t) => t, // successful? assign value to "updated_customer"
			Err(e) => bail!(e) // fail? throw error
		};

		// format info into a string
		let p = format!("Your account has been updated successfuly. New info: \n\n{:#?}", updated_account);
		self.set_display_text(p).draw(terminal); // Display updated user
		Ok(())
	}

	fn get_input(&mut self, terminal:&mut AppTerminal, input_type: char) -> Result<String> {
		self.set_hint_mode(2);
		loop {
			self.draw(terminal);
			if let Some(input_event) = input_event() {
				match (input_event, input_type) {
					(CustomInput::Char(c), t) =>{
						match t {
							'c' =>	self.input.push_str(&c.to_string()),
							't' =>	if self.input.len() == 0{
								self.input.push_str(&c.to_string())
							},
							'g' => self.input.push_str(&c.to_string()),
							_ => {}
						}
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
							'g' => self.input.push_str(&n.to_string()),
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
					(CustomInput::Up, _) => {
						self.scroll('u');
					}
					(CustomInput::Down, _) => {
						self.scroll('d');
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
					// _ => {}
				}
			}
		};
	}

	pub fn scroll_to_top(&mut self) {
		self.scroll = 0;
	}

	pub fn draw(&mut self, terminal:&mut AppTerminal) -> &mut Self{
		terminal.draw(|f| {
			super::ui(f, self)
		}).expect("Failed to draw interface");
		self
	}

	pub fn scroll(&mut self, direction: char) -> &mut Self{
		match direction {
			'u' => {
				self.scroll = self.scroll.saturating_sub(1);
				self.scrollbar_state = self.scrollbar_state.position(self.scroll as u16);
			},
			'd' => {
				self.scroll = self.scroll.saturating_add(1);
				self.scrollbar_state = self.scrollbar_state.position(self.scroll as u16);
			},
			_ => {}
		}

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
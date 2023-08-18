use super::{utils::{save_customer, read_database, overwrite_db}, models::{Customer, Account}};

const ADMINPASSWORD: &str = "admin123"; 

pub enum Events {}

impl Events {
	pub fn new_customer(name: String, pin_code: String) -> Result<(String, Customer), String> {
		let customers: Vec<Customer> = read_database();

		if customers.iter().any(|c| c.name.to_uppercase() == name.to_uppercase()) {
			return Err("Sorry this customer already exists".to_string());
		}

		let customer: Customer = Customer { pin_code, name, accounts: Vec::new() };
		if save_customer(customer.to_owned()).is_ok() {
			return Ok(("Customer saved".to_string(), customer));
		} else {
			return Err("Failed to save user".to_string());
		}
	}

	pub fn find_customer(name: &String, pin_code: &String) -> Option<(usize, Customer)> {
		let customers: Vec<Customer> = read_database(); // Read from the database
		let name = name.to_owned();
		let pin_code = pin_code.to_owned();

		match customers.iter().enumerate().find(|&(_, customer)| customer.name.trim().to_uppercase() == name.trim().to_uppercase() && customer.pin_code == pin_code) {
			Some((c_i, c)) => return Some((c_i, c.to_owned())),
			_ => None
		}	
	}

	pub fn deposit_money(customer_index: usize, customer: Customer, account_number: String, amount: u32) -> Result<u32, String> {
		let mut customers: Vec<Customer> = read_database(); // Read from the database
		// let customer = customers[customer_index];
		let mut customer = customer;
		match customer.deposit_into_account(account_number, amount) {
			Ok(new_balance) => {
				customers[customer_index] = customer; // Replace the customer in this index with the updated customer
				overwrite_db(customers); // Overwrite the db with this information
				Ok(new_balance)
			},
			Err(e) => {
				Err(e)
			}
		}
	}

	

	pub fn withdraw_money(customer_index: usize, customer: Customer, account_number: String, amount: u32) -> Result<u32, String> {
		let mut customers: Vec<Customer> = read_database(); // Read from the database
		// let customer = customers[customer_index];
		let mut customer = customer;
		match customer.withdraw_from_account(account_number, amount) {
			Ok(new_balance) => {
				customers[customer_index] = customer; // Replace the customer in this index with the updated customer
				overwrite_db(customers); // Overwrite the db with this information
				Ok(new_balance)
			},
			Err(e) => {
				Err(e)
			}
		}
	}

	pub fn close_bank_account(customer: Customer, customer_index: usize, account_number: String) -> Result<Customer, String>{
		let mut customers = read_database();
		let mut customer = customer;
		match customer.close_account(account_number) {
			Ok(_success_message) => {
				customers[customer_index] = customer.clone(); // Replace the customer in this index with the updated customer
				overwrite_db(customers); // Overwrite the db with this information
				Ok(customer)
			},
			Err(e) => {
				Err(e)
			}
		}
	}


	pub fn get_admin_info(admin_credentials: String) -> Result<Vec<Customer>, String> {
		if admin_credentials == ADMINPASSWORD {
			let customers: Vec<Customer> = read_database(); // Read from the database
			Ok(customers)
		} else {
			Err(String::from("INVALID CREDENTIALS! ACCESS DENIED!"))
		}
	}


	pub fn update_bank_account(customer: Customer, customer_index: usize, old_account_number: String, new_account_number: String) -> Result<Account, String> {
		let mut customers = read_database();
		let mut customer = customer.to_owned(); // Take ownership of the customer and update their account
		match customer.update_account(old_account_number, new_account_number) {
			Ok(account) => {
				customers[customer_index] = customer; // Replace the customer in this index with the updated customer
				overwrite_db(customers); // Overwrite the db with this information
				Ok(account)
			},
			Err(e) => {
				Err(e)
			}
		}
	}

	pub fn create_bank_account(customer_index: usize, account: (String, String)) -> Option<Account> {
		let account: Account = Account { account_number: account.0, account_type: account.1, balance:0 };
		let mut customers = read_database();
		customers[customer_index].accounts.push(account.clone());
	
		overwrite_db(customers);
		Some(account)
	}
	
}

pub fn get_customer(name: String, pin_code: String) -> Option<(usize, Customer)>{
	let customers: Vec<Customer> = read_database();
	match customers.iter().enumerate().find(|&(_, customer)| customer.name.to_uppercase() == name.to_uppercase() && customer.pin_code == pin_code) {
		Some((index, customer)) => Some((index, customer.to_owned())),
		_ => {None}
	}
}
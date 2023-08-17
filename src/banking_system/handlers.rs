use std::time::Duration;

// use anyhow::Ok;

use crate::interfaces::app;

use super::{utils::{print_prompt, save_customer, read_database, prompt, get_int_input, overwrite_db, goto_main_menu, empty_line, yes_or_no_decision}, models::{Customer, Account}};

const ADMINPASSWORD: &str = "admin123"; 

pub enum Events {}

impl Events {
	pub fn new_customer(name: String, pin_code: String) -> Result<(String, Customer), String> {
		let customers: Vec<Customer> = read_database();
		// let name = loop {
		// 	let input = prompt("");

		if customers.iter().any(|c| c.name.to_uppercase() == name.to_uppercase()) {
			// println!();
			return Err("Sorry this customer already exists".to_string());
			// return (Some(("Sorry, this customer already exists, try a different name: ").to_string()), None);
			// continue;
		}
		// };
		// let name = 
		// print_prompt("Enter your PIN code: ");
		// let pin_code =  get_int_input(Some(1000), 9999).to_string();

		let customer: Customer = Customer { pin_code, name, accounts: Vec::new() };
		if save_customer(customer.to_owned()).is_ok() {
			return Ok(("Customer saved".to_string(), customer));
		} else {
			return Err("Failed to save user".to_string());
		}
	}

	pub fn find_customer(name: &String, pin_code: &String) -> Option<(usize, Customer)> {
		let mut customers: Vec<Customer> = read_database(); // Read from the database
		let name = name.to_owned();
		let pin_code = pin_code.to_owned();

		match customers.iter().enumerate().find(|&(_, customer)| customer.name.trim().to_uppercase() == name.trim().to_uppercase() && customer.pin_code == pin_code) {
			Some((c_i, c)) => return Some((c_i, c.to_owned())),
			_ => None
		}

		// if customer.pin_code == String::new() {
		// 	None
		// } else {
		// 	Some((customer_index, customer))
		// }		
	}

	// pub fn get_customer_accounts(customer_index: usize) -> Option<()>{		
	// 	let mut customers: Vec<Customer> = read_database(); // Read from the database
	// 	if customer_index > customers.len() - 1 && customers[customer_index] == cu{
	// 		None
	// 	} else {
	// 		Some(())
	// 	}
	// }

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

		// // Search for customer
		// let (customer_index, customer) = get_customer(name, pin_code).unwrap_or((0, Customer {name: String::new(), pin_code: String::new(), accounts: Vec::new()}));

		// // Customer not found
		// if customer.name == String::new() {
		// 	return Err("It seems you have not registered with us. try registering first".to_string());
		// } else {
		// 	Ok(1)
		// }
		// match customer.deposit_into_account(selected_account_number, selected_amount) {
		// 	Ok(new_balance) => {
		// 		customers[customer_index] = customer; // Replace the customer in this index with the updated customer
		// 		overwrite_db(customers); // Overwrite the db with this information
		// 		println!("Your account has been credited successfully, your new balance is {}", new_balance);
		// 		empty_line();
		// 		return goto_main_menu();  // go to main menu
		// 	},
		// 	Err(e) => {
		// 		println!("{}", e); // Print any errors to stdout
		// 		return goto_main_menu();
		// 	}
		// }
		
		// Customer does not have an account, ask if they would like to create one and act accordingly
		// if customer.accounts.len() < 1 {create_account_prompt(&customers, customer_index); return;}

		// // If the customer has at least 1 account
		// println!("Your account(s): ");
		// for account in &customer.accounts { // list their accounts
		// 	println!("  - {:#?}", account);
		// }
		
		// empty_line();
		// let selected_account_number = prompt("Select the account number of the account you would like to deposit into: "); // Prompt the customer to select an account
		// // Customer selects invalid account, ask if they would like to create one and act accordingly
		// if !customer.accounts.iter().any(|acc| acc.account_number == selected_account_number) {create_account_prompt(&customers, customer_index); return;}

		// print_prompt("Enter the amount you would like to deposit: "); // Prompt them for an amount
		// let selected_amount = get_int_input(None, 50000);

		// let mut customer = customer.to_owned(); // Take ownership of the customer and deposit into their account

	}


	pub fn withdraw_money() {
		let name = prompt("Enter your name: ");
		let pin_code = prompt("Please enter your PIN code for verification: ");
		let mut customers: Vec<Customer> = read_database(); // Read from the database

		// Search for customer
		let (customer_index, customer) = get_customer(name, pin_code).unwrap_or((0, Customer {name: String::new(), pin_code: String::new(), accounts: Vec::new()}));

		if customer.name == String::new() {return;}
		
		// Customer does not have an account, ask if they would like to create one and act accordingly
		if customer.accounts.len() < 1 {create_account_prompt(&customers, customer_index); return;}

		// Customer found
		println!("Your account(s): ");
		for account in &customer.accounts { // list their accounts
			println!("  - {:#?}", account);
		}
		empty_line();
		let selected_account_number = prompt("Select the account number of the account you would like to withdraw from: "); // Prompt the customer to select an account
		// Customer selects invalid account, ask if they would like to create one and act accordingly
		if !customer.accounts.iter().any(|acc| acc.account_number == selected_account_number) {create_account_prompt(&customers, customer_index); return;}

		print_prompt("Enter the amount you would like to withdraw: "); // Prompt them for an amount
		let selected_amount = get_int_input(None, 50000);
		let mut customer = customer.to_owned(); // Take ownership of the customer and deposit into their account
		match customer.withdraw_from_account(selected_account_number, selected_amount) {
			Ok(new_balance) => {
				customers[customer_index] = customer; // Replace the customer in this index with the updated customer
				overwrite_db(customers); // Overwrite the db with this information
				println!("Your account has been debited successfully, your new balance is {}", new_balance);
				empty_line();
				return goto_main_menu();  // go to main menu
			},
			Err(e) => {
				println!("{}", e); // Print any errors to stdout
				return goto_main_menu();
			}
		}
	}

	pub fn close_bank_account() {
		let name = prompt("Enter your name: ");
		let pin_code = prompt("Please enter your PIN code for verification: ");
		let mut customers: Vec<Customer> = read_database(); // Read from the database

		// Search for customer
		let (customer_index, customer) = get_customer(name, pin_code).unwrap_or((0, Customer {name: String::new(), pin_code: String::new(), accounts: Vec::new()}));

		if customer.name == String::new() {return;}

		if customer.accounts.len() < 1 {eprintln!("No accounts available to close");	return}

		// Customer found
		println!("Your account(s): ");
		for account in &customer.accounts { // list their accounts
			println!("  - {:#?}", account);
		}
		empty_line();
		let selected_account_number = prompt("Select the account number of the account you would like to close: "); // Prompt the customer to select an account
		// Customer selects invalid account, ask if they would like to create one and act accordingly
		if !customer.accounts.iter().any(|acc| acc.account_number == selected_account_number) {eprintln!("No such account found"); return goto_main_menu();}
		
		let mut customer = customer.to_owned(); // Take ownership of the customer and deposit into their account
		match customer.close_account(selected_account_number) {
			Ok(success_message) => {
				customers[customer_index] = customer; // Replace the customer in this index with the updated customer
				overwrite_db(customers); // Overwrite the db with this information
				println!("{}", success_message);
				empty_line();
				return goto_main_menu();  // go to main menu
			},
			Err(e) => {
				eprintln!("{}", e); // Print any errors to stdout
				return goto_main_menu();
			}
		}
	}

	pub fn get_account_balances() {
		let name = prompt("Enter your name: ");
		let pin_code = prompt("Please enter your PIN code for verification: ");
		let customers: Vec<Customer> = read_database(); // Read from the database

		// Search for customer
		let (customer_index, customer) = get_customer(name, pin_code).unwrap_or((0, Customer {name: String::new(), pin_code: String::new(), accounts: Vec::new()}));

		// Customer not found
		if customer.name == String::new() {return;}
		
		// Customer does not have an account
		if customer.accounts.len() < 1 {create_account_prompt(&customers, customer_index); return;}

		println!("Your account(s): ");
		for account in &customer.accounts { // list their accounts
			println!("  - {:#?}", account);
		}
		empty_line();
		return goto_main_menu();
	}

	pub fn get_admin_info() {
		if prompt("Enter admin credentials: ") == ADMINPASSWORD {
			println!("Access granted, getting customers");
			std::thread::sleep(Duration::new(1, 0));
			let customers: Vec<Customer> = read_database(); // Read from the database
			println!("CUSTOMERS\n {:#?}", customers);
			std::thread::sleep(Duration::new(1, 0));
			return goto_main_menu();
		} else {
			println!("Access denied");
			return goto_main_menu();
		}
	}


	pub fn update_bank_account() {
		let name = prompt("Enter your name: ");
		let pin_code = prompt("Please enter your PIN code for verification: ");
		let mut customers: Vec<Customer> = read_database(); // Read from the database
		// Search for customer
		let (customer_index, customer) = get_customer(name, pin_code).unwrap_or((0, Customer {name: String::new(), pin_code: String::new(), accounts: Vec::new()}));

		if customer.name == String::new() {return;} // Customer not found
		
		// Customer does not have an account, ask if they would like to create one and act accordingly
		if customer.accounts.len() < 1 {eprintln!("Account not found"); return goto_main_menu();}
		
		println!("Your account(s): ");
		for account in &customer.accounts { // list their accounts
			println!("  - {:#?}", account);
		}
		
		empty_line();
		let old_account_number = prompt("Select the account number of the account you would like to update: "); // Prompt the customer to select an account
		// Customer selects invalid account, ask if they would like to create one and act accordingly
		if !customer.accounts.iter().any(|acc| acc.account_number == old_account_number) {create_account_prompt(&customers, customer_index); return;}

		println!("Enter the new account number you would like");
		let new_account_number = get_int_input(Some(1000), 9999).to_string();

		if customer.accounts.iter().any(|acc| acc.account_number == new_account_number) {
			println!("The account number you entered already exists");
			return
		}
		let mut customer = customer.to_owned(); // Take ownership of the customer and update their account
		match customer.update_account(old_account_number, new_account_number) {
			Ok(success_message) => {
				customers[customer_index] = customer; // Replace the customer in this index with the updated customer
				overwrite_db(customers); // Overwrite the db with this information
				println!("{}", success_message);
				empty_line();
				return goto_main_menu();  // go to main menu
			},
			Err(e) => {
				eprintln!("{}", e); // Print any errors to stdout
				return goto_main_menu();
			}
		}
	}	
}


fn create_account(customers: Vec<Customer>, customer_index: usize) {
	let mut customers = customers;
	let account_type = loop {
		match prompt("Select account type(C/S): ").to_uppercase().as_str() {
			"C"	=> break String::from("current"),
			"S" => break String::from("savings"),
			_ => println!("Invalid input")
		}
	};
	println!("Input a 4-digit account number");
	let account_number= get_int_input(Some(1000), 9999).to_string();
	let account: Account = Account { account_number: account_number.to_owned(), account_type, balance: 0 };

	let index = customer_index;
	let existing_accounts = &customers[index].accounts;

	if existing_accounts.len() > 0 && existing_accounts.iter().any(|x| x.account_number == account_number) {
		eprintln!("An account with the given account number already exists");
		return goto_main_menu()
	}	
	customers[customer_index].accounts.push(account);

	overwrite_db(customers);
	println!("\n Account saved! Thank you \n");
	std::thread::sleep(Duration::new(1, 0));
	return goto_main_menu();
}

pub fn get_customer(name: String, pin_code: String) -> Option<(usize, Customer)>{
	let customers: Vec<Customer> = read_database();
	match customers.iter().enumerate().find(|&(_, customer)| customer.name.to_uppercase() == name.to_uppercase() && customer.pin_code == pin_code) {
		Some((index, customer)) => Some((index, customer.to_owned())),
		_ => {None}
	}
}

fn create_account_prompt(customers: &Vec<Customer>, customer_index: usize) {
	if yes_or_no_decision("No account found, would you like to create an account?(Y/N): ") {
		println!("Yay! Let's get started then.");		
		return create_account(customers.to_owned(), customer_index)
	} else {
		return goto_main_menu();
	}
}
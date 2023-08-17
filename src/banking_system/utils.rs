use std::{io::{Write, BufWriter}, fs::File, time::Duration};

use super::models::Customer;

// use crate::banking_system::

// Macros
macro_rules! read_t{
	($t:tt) => {{
			let mut temp = String::new();
			std::io::stdin().read_line(&mut temp).expect("fail");
			temp.trim().parse::<$t>()
	}};
}

//Constants
const FILE_PATH: &str = "database.json";

pub fn get_int_input( lower_bound: Option<u32>, upper_bound: u32) -> u32 {
	let lower_bound = lower_bound.unwrap_or(0);
	let res = loop {
		match read_t!(u32) {
			Ok(i) => if i > upper_bound || i < lower_bound {
				print_prompt("Invalid selection, try again: ")
			} else {
				break i
			},
			_ => {            
				print_prompt("Invalid input, try again: ");
			}
		}
	};

	res
}


pub fn get_string_input() -> String {
	let res = read_t!(String).unwrap();
	res
}

pub fn print_prompt(prompt: &str) {
	print!("{}", prompt);
	std::io::stdout().flush().unwrap();
}

pub fn prompt(prompt: &str) -> String {
	print!("{}", prompt);
	std::io::stdout().flush().unwrap();
	get_string_input()
}

pub fn goto_main_menu() {
	std::thread::sleep(Duration::new(1, 0));
	empty_line();
	println!("Redirecting to main menu...\n");
	std::thread::sleep(Duration::new(1, 0));
	// main()
}

pub fn empty_line() {
	println!("");
}

pub fn save_customer(customer: Customer) -> Result<bool, String>{
	let mut customers= read_database();

	customers.push(customer);
	overwrite_db(customers);
	Ok(true)
}

pub fn read_database() -> Vec<Customer> {
	let file =  if let Ok(file_contents) = std::fs::read_to_string(FILE_PATH) {
		file_contents
	} else {
		File::create(FILE_PATH).unwrap(); // create the file if it does not exist
		String::from("[]")
	};
	let customers: Vec<Customer> =  match serde_json::from_str(file.as_str()) {
		Ok(i) => i,
		_ => {Vec::new()}
	};

	customers
}

pub fn overwrite_db(info: Vec<Customer>) {	
	let db = File::create(FILE_PATH).unwrap();
	let mut writer = BufWriter::new(db);
	serde_json::to_writer(&mut writer, &info).unwrap();
}

pub fn yes_or_no_decision(input_prompt: &str) -> bool {
	match prompt(input_prompt).to_uppercase().as_str() {
		"Y" => {
			return true
		},
		"N" => {
			print!("Alright, thank you");
			return false
		},
		_ => {
			println!("Understood, have a nice day.");
			return false
		}
	}
}


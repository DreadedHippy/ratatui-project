use std::{io::BufWriter, fs::File};

use super::models::Customer;

//Constants
const FILE_PATH: &str = "database.json";

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


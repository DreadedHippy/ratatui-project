// use std::io::Write;
mod models;
mod utils;
pub mod handlers;

use handlers::Events;



pub fn start_bank(selection: u8) {
    match selection {
        1 => {
            // Events::new_customer();
        },
        2 => {
            // Events::deposit_money();
        },
        3 => {
            Events::withdraw_money();
        },
        4 => {
            Events::get_account_balances();
        },
        5 => {
            Events::get_admin_info();
        },
        6 => {
            Events::close_bank_account()
        },
        7 => {
            Events::update_bank_account();
        },
        8 => {
            println!("Thank you for banking with us");
            return;
        },
        _ => {}
    }
    // println!("{}", input);
}

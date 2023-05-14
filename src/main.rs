use std::fmt;

mod cliio;
mod executor;
mod types;

use crate::cliio::{get_inp, fflush};
use crate::types::{Employee};
use crate::executor::{add_employee};

fn main() {
    let mut employees: Vec<Employee> = vec![];
    loop {
   		print!("employee > ");
   		fflush();
    	let command = get_inp();
    	let mut words = command.split_whitespace();
    	match words.nth(0).unwrap().to_lowercase().as_str() {
    		"add" => {
    			add_employee(&mut employees, command)
    				.unwrap_or_else(|err| {
	    				println!("{:?}", err);
	    			});
    		},
    		"display" => println!("{:?}", employees),
    		_ => continue,
    	};
    }
}

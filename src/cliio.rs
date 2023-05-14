use std::io::Write;
use std::io;

pub fn get_inp() -> String {
	let mut inp = String::new();
	io::stdin()
    	.read_line(&mut inp)
    	.expect("Failed to read line");
    inp.clone()
}

pub fn fflush() {
	io::stdout().flush().unwrap();
}
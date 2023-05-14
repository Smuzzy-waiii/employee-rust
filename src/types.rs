#[derive(Debug)]
pub enum Department {
	Sales,
	Operations,
	Engineering,
}

#[derive(Debug)]
pub struct Employee {
	pub name: String,
	pub dept: Department,
}
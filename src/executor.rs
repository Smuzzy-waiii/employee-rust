use crate::types::{Employee, Department};

pub fn add_employee(employees: &mut Vec<Employee>, command: String) -> Result<(), &str> {
	let mut words = command.split_whitespace();
	let emp_name = words.nth(1).unwrap();
	let emp_dept = words.nth(1).unwrap();
	let emp_dept = match emp_dept.to_lowercase().as_str() {
		"sales" => Department::Sales,
		"operations" => Department::Operations,
		"engineering" => Department::Engineering,
		_ => {
			return Err("Invalid Dept Entered!");
		}
	};
	let employee = Employee {
		name: emp_name.to_string(),
		dept: emp_dept,
	};
	employees.push(employee);
	return Ok(());
}
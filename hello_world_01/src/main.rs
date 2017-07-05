// This is the simple practice of rust-by-example
// Ch1 - Hello World

fn main() {
    println!("Hello, world!");
	
	println!("\nThe usage of #[derive(Debug)]\n");
	print_user();

	
	println!("\nThe usage of impl display trait\n");
	print_user_from_display();
	
	println!("\nThe usage of format traits\n");
	print_format_traits();
}

// 1.2.1 Debug

#[derive(Debug)]
struct User {
	name: String,
	email: String,
	id: i32,
}

fn print_user() {
	// initialized a structure
	let jon = User{
		name: String::from("Jon"),
		email: String::from("Jon@123.com"),
		id: 123
	};
	
	// print struct as plain text
	println!("The plain text output of User : \n {:?} ", jon);
}

// 1.2.2 Display
// implement the fmt::Display trait

use std::fmt;

impl fmt::Display for User {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "name : {}\nemail: {}\nid: {}", self.name, self.email, self.id)
	}
}

fn print_user_from_display() {
	// initialized a structure
	let jon = User{
		name: String::from("Jon"),
		email: String::from("Jon@123.com"),
		id: 123
	};

	println!("{}", jon);
}

// 1.2.3 Format
// std::fmt format traits

fn print_format_traits() {
	let a_number = 1532;
	
	println!("Decimal : {}", a_number);
	println!("Octal: 0o{:o}", a_number);
	println!("Hex: 0x{:x}", a_number);
}

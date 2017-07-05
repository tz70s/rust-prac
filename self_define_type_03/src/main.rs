// simple practice of rust-by-example
// Ch3 of self defined types

// the implementation of linked list
// using enum

use List::*;

enum List {
	Cons(i32, Box<List>),
	Nil,
}

impl List {
	fn new() -> List {
		Nil
	}
	
	fn append(self, elem: i32) -> List {
		Cons(elem, Box::new(self))
	}

	fn len(&self) -> u32 {
		match *self {
			Cons(_, ref tail) => 1 + tail.len(),
			Nil => 0,
		}
	}

	fn stringify(&self) -> String {
		match *self {
			Cons(head, ref tail) => {
				format!("{} -> {}", head, tail.stringify())
			},
			Nil => {
				format!("nil")
			},
		}
	}
}

fn main() {
    println!("Hello, world!");

	let mut list = List::new();
	
	list = list.append(1);
	list = list.append(2);
	list = list.append(3);
	list = list.append(4);

	println!("List len : {}", list.len());
	println!("List : {}", list.stringify());
}

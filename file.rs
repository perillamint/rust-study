/**
 * author: KBuild<qwer7995@gmail.com>
 * desc: file save example
 */

#![feature(io)] // use old_io
#![feature(path)] // use file path
#![feature(core)] // use as_slice

use std::old_io::{File, Append, ReadWrite};
use std::old_io::stdin;
use std::str::FromStr;

struct Book {
	name : String,
	cost : u32,
}

impl Book {
	fn new(_name: String, _cost: u32) -> Book{
		Book{ name : _name, cost : _cost}
	}

	fn get_name(&self) -> &String {
		&self.name
	}

	fn get_cost(&self) -> u32 {
		self.cost
	}

	fn to_string(&self) -> String {
		let mut ret = String::new();
		ret.push_str(self.name.as_slice());
		ret.push_str(",");
		ret.push_str(self.cost.to_string().as_slice());
		ret
	}
}


fn main() {

	let p = Path::new("data.txt");

	let mut file = match File::open_mode(&p, Append, ReadWrite) {
		Ok(f) => f,
		Err(e) => panic!("file error: {}", e),
	};

	print!("Input Book's name : ");
	let inputn = stdin().read_line().ok().expect("Fail to read line");
	let name: String = FromStr::from_str(inputn.trim()).unwrap();

	print!("Input Book's cost : ");
	let inputc = stdin().read_line().ok().expect("Fail to read line");
	let cost: u32 = FromStr::from_str(inputc.trim()).unwrap();

	let book = Book::new(name, cost);
	println!("{}, {} saved", book.get_name(), book.get_cost());
	file.write_str(book.to_string().as_slice());
	file.write_str("\n");

/*
 	let book1 = Book::new("옥상의 민들레꽃", 10000);
	let book2 = Book::new("동의보감", 300000);


	file.write_str(book1.to_string().as_slice());
	file.write_str("\n");
	file.write_str(book2.to_string().as_slice());
	file.write_str("\n");
*/
}

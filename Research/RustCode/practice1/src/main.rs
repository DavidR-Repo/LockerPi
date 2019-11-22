use std::io;
use std::env; // get command line input

fn main() {
	let functs = [
		variable_basics,
    	command_line_args,
		ownership1,
		multiple_ownership,
		slices,
		control_flow,
		structs,
		methods,
		enums,
		match_,
		if_let,
	];
	
	let to_run = functs[functs.len() - 1];
	to_run();
}


fn variable_basics(){
    // the '!' denotes a macro
    println!("Welcome to rust, what is your name?");
	
	// instantiate a mutable string variable with implicit type inference
	let mut str_var = String::new();
	
	// standard input
	// "could not read line" message if error is thrown
	io::stdin().read_line(&mut str_var)
        .expect("could not read line");
	
	// placeholder curly brace
	println!("Hello {}!", str_var);
	
	let tuple_of_one : (u8,) = (5,);
	let nested_tuple_primatives = (
			(true, false, "bool"),
			('c', "char", "four byte Unicode Scalar Value"),
			(1.0, 1.0f64, "f32 f64", "floats"),
			(1, 1i8, "u16, i128, usize...", "8, 16, 32, 64, 128, and archetecture-specific bit size"),
						);
		// note: integer overflow thows an error in debug and wrapps w/out error in release
		
	let tuple_of_number_literals = (
			10000, 10_000, // underscore is ignored, use for readability
			0xff, // hex
			0o74, // octal
			0b1111_0000, // binary
			b'A' // byte (u8)
			);
	
	println!("{:?}",tuple_of_one);
	println!("{:?}", nested_tuple_primatives);
	println!("{:?}", tuple_of_number_literals);
	println!("Tuple must be a max length of 12 to use 'println!'");
}

fn command_line_args() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

fn ownership1() {
	let x = String::from("A string");

	// using 'let y = x;' would result in an error
	//   because contents of y are moved to x
	//   and y in invalidated
	let y = x.clone();
	println!("x = {},  y = {}", x, y);

	let i = 5u8;

	// this is fine because k does not implement drop (to deallocate)
	// k is a primative, or a constant size collection of primatives
	let k = i;
	println!("i = {},  k = {}", i, k);

	// note: a variable returned from a function
	// moves ownership to the function's return
	let mut name = String::from("Me");
	println!("name is {}",name);
	let who = String::from("David");
	name = ownership2(who);
	println!("name is now {}", name);

	let mut other_name = String::from("Joe");
	ownership3(&name, &mut other_name);
	println!("name is still {}", name);
	println!("But other_name is now {}",other_name);
}

fn ownership2 (s: String) -> String {
	println!("Hello {}!",s);
	s
}

fn ownership3 (s1: &String, s2 : &mut String) {
	// reference parameter = borrowing
	println!("s1({}) and s2({}) are used by reference", s1, s2);
	s2.push_str("!");
	println!("s2 is changed to {}", s2);
}

fn multiple_ownership () {
	let mut s = String::from("Hey there!");
	{
		let x = &s; // x is not mutable
		let y = &s; // y is also not mutable
		println!("x: {}  y: {}", x, y);

		// either one mutable reference or any number of immutable references
		// only allowed because x and y are no longer used
		// when used in println!
		let z = &mut s; // 'let k = &mut s;' would fail to compile
		println!("z: {}",z);
	}
	// z is now out of scope
	let k = &mut s;
	println!("k: {}", k);
}

fn slices () {
	let s = String::from("012345");
	let _0_to_2 = &s[0..3];
	let to_3 = &s[..4];
	let from_3 = &s[3..];
	println!("0 to 2 : {}",_0_to_2);
	println!("to 3 : {}", to_3);
	println!("from 3 : {}", from_3);
}

fn control_flow() {
	let n = 3;
	
	if n < 5 {
	// condition must be bool (no implicit conversion)
		println!("{} is less than 5!", n);
	} else if n == 5 {
		println!("{} is equal to 5!", n);
	} else {
		println!("{} is greater than 5!", n);
	}
	
	// conditional instantiation (all branches must return the same type)
	let value = if n == 3 {
		String::from("Three") // no semicolon ==> branch return (expression)
	} else {
		String::from("Not Three")
	}; // semicolon ==> end of let statement
	println!("{}", value);


	loop {
		println!("Continue?(y/n)");
		let mut answer = String::new();
		io::stdin().read_line(&mut answer)
	        .expect("could not read line");
		if &answer[0..1] != "y" &&
			&answer[0..1] != "Y" {
				println!("you answered:{}", &answer[0..1]);
				break;
		}	// (else continue)
	}

	let mut count = 0;
	let result = loop {
		count += 1;
		if count >= 10 {
			break count * 10; // break & send value to 'result' variable
		}
	}; // statement because loop sends value to 'result'
	println!("Ten times result = {}", result);


	while count < 100 {
		count *= 2;
		println!("count = {}", count);
	}


	let an_array = [1,2,3,4,5,6,7,8,9,10];
	for item in an_array.iter() {
		print!("({}) ", item);
	}
	println!();

	for item in (-2..8).rev() {
		print!("({}) ", item);
	}
	println!();
	//let another_array = [1,3,5,6,7,9,11,13,15];
	for item in an_array[..5].iter().rev() {
		print!("({}) ", item);
	}
	println!();
}


fn structs(){

	// structs: all or nothing mutability
	#[derive(Debug)] // allows printing behavior (use debug print"{:?}" or pretty-print"{:#?}")
	struct Color {
		color_name: String,
		color_opacity: f32,
	};

	#[derive(Debug)]
	struct Blob {
		name: String,
		id: u16,
		color: Color, // see CH 10 (rustup doc) for reference fields (&Color instead of Color)
		mass: u16,
	};
	let color_name = String::from("Red");
	let see_through: f32 = 0.255;

	let blobby = Blob {
		name: String::from("Goopy"),
		id: 10,
		color: Color {
			color_name,  // all field names must match type, name, and order of struct declaration
			color_opacity: see_through, // 'field init shorthand' syntax: any number of fields
		},
		mass: 255,
	};
	println!("name:{}  id:{}  color:{}  color opacity:{}  size:{}",blobby.name, blobby.id, blobby.color.color_name, blobby.color.color_opacity, blobby.mass);

	let blob = Blob {
		mass: blobby.mass * 100,
		..blobby
	};
	println!("name:{}  id:{}  color:{}  color opacity:{}  size:{}",blob.name, blob.id, blob.color.color_name, blob.color.color_opacity, blob.mass);


	// tuple struct
	#[derive(Debug)]
	struct ColorStruct(i8,i8,i8); // not the same type as a '(i8,i8,i8)' tuple or other tuple struct
	let black = ColorStruct(0,0,0);
	println!("black:\n{:?}",black);

	#[derive(Debug)]
	struct AnObjectWithoutData();
	let a_trait = AnObjectWithoutData {};
	println!("a trait:\n{:#?}",a_trait);
}

fn methods(){
	// (member functions)
	#[derive(Debug)]
	struct Rectangle {
		width: u32,
		height: u32,
	}

	impl Rectangle {
		fn area(&self) -> u32 {
			self.width * self.height
		}

		fn perimeter(&self) -> u32 {
			(self.width + self.height) * 2
		}

		fn fits_inside(&self, other: &Rectangle) -> bool {
			other.width > self.width && other.height > self.height
		}
		
	}

	let r = Rectangle {
		width: 15,
		height: 100,
	};

	let r2 = Rectangle {
		width: 16,
		height: 99
	};

	let r3 = Rectangle {
		width: 16,
		height: 101
	};

	println!("Rectangle r - area: {}   perimeter: {}",r.area(), r.perimeter());
	println!("r: {:?}", r);
	println!("r2: {:?}", r2);
	println!("r3: {:?}", r3);
	println!("r fits in r2: {}", r.fits_inside(&r2));
	println!("r fits in r3: {}", r.fits_inside(&r3));

	// note: r.area() == (&r).area()  'automatic referencing and dereferencing
	// 		 mutability is also determined implicitly


	// Associated Functions (don't take &self) (like: static function)
	// multiple 'impl' allowed
	impl Rectangle {
		fn square(side: u32) -> Rectangle {
			Rectangle { width: side, height: side }
		}
	}
	println!("Creating a square...\n\t{:?}", Rectangle::square(55));
}


fn enums() {
	#[derive(Debug)]
	enum IpAddressType {
		V4,
		V6,
	}
	println!("IP address types: {:?} and {:?}", IpAddressType::V4, IpAddressType::V6);

	#[derive(Debug)]
	enum IpAddress {
		V4(u8,u8,u8,u8),
		V6(String),
	}
	println!("IP address types with data: {:?} and {:?}", IpAddress::V4(10,0,0,1), IpAddress::V6(String::from("256AFF920JGELS925")));

	impl IpAddress {
		fn validate(address: &IpAddress) -> bool {
			false // always false
		}
	}
	let adrs = IpAddress::V6(String::from("2047392023948"));
	println!("{:?} {}",adrs, IpAddress::validate(&adrs));

	// enum - std::option::Option, like null in other langs (but safer)
	// forces conversion from Option<T> to T before use (less likely to assume non-null)
	// implicitly in scope just use 'Option<T> = None' or use Some(T)'
	/*
	enum Option<T> {
		Some(T),
		None,
	}
	*/

}

fn match_() {
	enum Coin {
		Penny,
		Nickle,
		Dime,
		Quarter,
	}

	fn value(coin: Coin) -> f32 {
		// enforces handling all Coin cases separated by commas (exhaustive)
		// like a special switch/case statement
		match coin {
			// pattern  => expression,
			Coin::Penny => 0.01,
			Coin::Nickle => 0.05,
			Coin::Dime => {
				let d = Coin::Nickle *2;
				d
			},
			Coin::Quarter => 0.25,
		}
	}

	fn and_one(num: Option<i32>) -> Option<i32> {
		match num {
			// adds one to num, enforces 'None' check
			None => None,
			Some(i) => Some(i + 1),
		}
	}

	let x = 0u8;
	match x {
		3 => println!("three"),
		100 => println!("One hundo"),
		5 => println!("fiver"),
		255 => println!("last num"),
		_ => (), // '()' = unit value: do nothing
				// '_' = placeholder: "anything not yet matched"
	}
}

fn if_let() {
	let x = Some(3u8);
	// match version
	match x {
		Some(3) => println!("you found THREE!!!"),
		_ => (),
	}

	// if-let equivalent in the case of 
		// pattern    expression
	if let Some(24) = x {
		println!("24? Ya, that's right");
	} else /* optional */ {
		println!("No way Jose");
	}
}
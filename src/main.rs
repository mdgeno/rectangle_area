use std::io;

#[derive(Debug)]
struct Rectangle{
	width: u32,
	height: u32,
}

impl Rectangle{
	fn area(self: &Self) -> u32{
		self.width*self.height
	}

	fn hold(self: &Self, other: &Rectangle) -> bool{
		self.width>other.width && self.height>other.height
	}
}

fn main(){

	let mut user_input = String::new();
	io::stdin().read_line(&mut user_input).expect("enter valid input");

	println!("user input was: {}", &user_input);

	let rect1 = Rectangle{
		width: 30,
		height: 50,
	};	
	let rect2 = Rectangle{
		width: 10,
		height: dbg!(40*1),
	};
	let rect3 = Rectangle{
		width: 60,
		height: 45,
	};

	println!("Can rect1 hold rect2? {}", rect1.hold(&rect2));
	println!("Can rect1 hold rect3? {}", rect1.hold(&rect3));

	println!(" ");

	println!("derived standard format of rectangle 1 variable {rect1:?}");
	println!("derived standard format of rectangle 2 variable {rect2:?}");
	println!("derived pretty print format of rectangle 3 variable {rect3:#?}");

	println!(" ");

	dbg!(rect2.height);
}
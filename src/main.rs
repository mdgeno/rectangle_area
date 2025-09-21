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

impl Rectangle{
	fn square(size: u32) -> Self{
		Self{
			width: size,
			height: size,
		}
	}
}

fn main(){

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

	println!("Can rect1 hold rect2: {}", rect1.hold(&rect2));
	println!("Can rect1 hold rect3: {}", rect1.hold(&rect3));

	println!("");

	println!("Testing rect1 variable with derive trait debug: {rect1:?}");
	println!("Testing rect2 variable with derive trait debug: {rect2:?}");

	println!("");

	dbg!(rect2.height);

	println!("");

	loop{
		println!("please enter size of square");

		let mut user_in = String::new();
		io::stdin().read_line(&mut user_in).expect("enter valid input");
		
		let user_in: u32 = match user_in.trim().parse(){
					Ok(num) => num,
					Err(_) => continue,
				   }; 
	
		let new_square = Rectangle::square(user_in);

		println!("Checking the value of the new_square variable: {new_square:#?}");
	
		break;
	}   
}

//test


























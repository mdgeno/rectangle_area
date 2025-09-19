//create rectangle struct.
//apply derived debug features to test how the vairables work.
#[derive(Debug)]
struct Rectangle{
	width: u32,
	height: u32,
}

//create an implementation block for the area method
impl Rectangle{

	//when creating a method in the implementation block, 
	//type in the long form of the required initial self 
	//parameter.
	fn area(self: &Self) -> u32{
		self.width*self.height
	}	
		
}

//create a main function
fn main(){

	//initialise an instance of the Rectangle struct.
	let rect1 = Rectangle{
		width: dbg!(30*2),
		height: 50,
	};	

	//print line macro the text ' ' and utilising the 
	//implemented method of the Rectangle struct.
	println!("The area of the rectangle is {}", rect1.area());
	
	println!(" ");

	//use normal :? output and :#? pretty print
	println!("the value of rect1 is {rect1:?}");
	println!("the value of rect1 with pretty print {rect1:#?}");

	//use the dbg! macro
	dbg!(rect1.width);
}










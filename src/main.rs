#[derive(Debug)]    //step 1 for debugging structs
struct Rectangle{
	width: u32,
	height: u32,
}

fn main(){

	let width_val = 30 *2;

	let name_val = String::from("recta");

	let rect1 = Rectangle{
		width: dbg!(width_val), //step 2 if dbg!
		height: 50,
	};

	println!("The area of the rectangle is {rect1:?}");  //step 2

	println!("The area of the rectangle is {rect1:#?}");   //or step 2

	dbg!(&rect1);  //another dbg! step

}
/*
fn area(rectangle: &Rectangle) -> u32{
	rectangle.width*rectangle.height
}
*/ 
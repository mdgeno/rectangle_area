fn main(){

/*	let width1 = 30;
	let height1 = 50;
	let area1 = area(width1, height1);

	println!("The area of the rectangle is {area1} square pixels");
*/
//Refactor with tuple

	let tuple_rectangle: (u32, u32) = (30, 50);

	println!("The area of the rectangle is {}", area(tuple_rectangle));

}

/*
fn area(w: u32, h: u32) -> u32{
	w*h
}   */


//Refactor with tuple
fn area(tuple_rectangle: (u32, u32)) -> u32{
	tuple_rectangle.0*tuple_rectangle.1
}
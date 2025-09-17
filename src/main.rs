fn main(){

	let width1 = 30;
	let height1 = 50;
	let area1 = area(width1, height1);

	println!("The area of the rectangle is {area1} square pixels");

}

fn area(w: u32, h: u32) -> u32{
	w*h
}
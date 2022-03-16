
pub fn run(){

	let outer_x = "This is outer scope var";
	println!("{}",outer_x);
	{

		let inner_x = "This is inner scope var";
		println!("{}",inner_x);

	}

	let spaces = "  ";
	println!("{}",spaces);
	let spaces = spaces.len();

	println!("{}",spaces);


}
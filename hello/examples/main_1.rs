use std::convert::TryInto;

fn add(i: i32, j: i32) -> i32 {
	i + j
}

fn main() {
	let a = 10;
	let b: i32 = 20;
	let c = 30i32;
	let d = 30_i32;
	let e = add(add(a, b), add(c, d));

	println!("(a + b) + (c + d) = {}", e);

	let f: i32 = 10;
	let g: u16 = 100;

	if f < (g as i32) {
		println!("Ten is less than one hundred.");
	}

	let g_ = g.try_into().unwrap();

	if f < g_ {
		println!("Ten is less than one hundred.");
	}
}

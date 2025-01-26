
fn main() {
	let a: f32 = 1.0;
	let b: f32 = -10.0;
	let c: f32 = 4.0;

	let d: f32 = (b * b) - (4.0 * a * c);

	println!("D = d^2 + 4ac = {}", d);
}

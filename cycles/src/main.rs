fn main() {
	let collection = [1, 2, 3, 4, 5];

	for i in 0..collection.len() {
		let item = collection[i];
		println!("collection[{}]={}", i, item);
	}

	for item in &collection {
		println!("Item: {}", item);
	}
}

fn main() {
	println!("Hello, world!");

	let x = vec![1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1];

	let y = x
		.iter()
		.map(|x| x.to_string())
		.collect::<Vec<String>>()
		.join("");

	// dbg!(y);

	let mut z = y.split('0').collect::<Vec<&str>>();

	z.sort(); // O(n log_n) worst case

	let a = z.last().unwrap().len();

	dbg!(a);

	// z.last()
}

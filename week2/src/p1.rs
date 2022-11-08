fn main() {
	let s1: Vec<char> = "abCd".chars().collect(); // O(n)
	let s2: Vec<char> = "ffffabCdCefg".chars().collect(); // O(n)

	let mut ptr = 0;

	// O(n)
	for (i, x) in s2.iter().enumerate() {
		println!("ptr -> {}", ptr);
		// O(1)
		match s1.get(ptr) {
			Some(c) if c == x => ptr += 1,
			None => {
				println!("{}", i - s1.len());
				return;
			}
			_ => ptr = 0,
		}
	}
}

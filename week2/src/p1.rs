fn main() {
	// have to do this as rust chars are unicode scalar values
	let s1: Vec<char> = "abCd".chars().collect(); // O(n)
	let s2: Vec<char> = "aaabCdabCdabCdabCdabCdabCdabCd".chars().collect(); // O(n)

	let mut ptr = 0;

	// O(n)
	for (i, x) in s2.iter().enumerate() {
		// O(1)
		if ptr == s1.len() {
			println!("{}", i - s1.len());
			ptr = 0;
		}

		// O(1)
		if let Some(c) = s1.get(ptr) {
			if c == x {
				ptr += 1;
			} else if *x == s1[0] {
				ptr = 1;
			} else {
				ptr = 0;
			}
		}
	}

	// O(n) * O(1) = O(n)
}

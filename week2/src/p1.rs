fn main() {
	substring::find_substrings("abCd", "aaabCdfffabCdsdkfabCd");
}

pub mod substring {
	pub fn find_substrings(s1: &str, s2: &str) {
		// have to do this as rust chars are unicode scalar values
		let s1: Vec<char> = s1.chars().collect(); // O(n)
		let s2: Vec<char> = s2.chars().collect(); // O(n)

		let mut ptr = 0;

		// O(n)
		for (i, x) in s2.iter().enumerate() {
			let l = s1.len();

			// O(1)
			if ptr == l || (ptr == l - 1 && *x == s1[l - 1]) {
				println!("{}", i - l);
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
	}
}

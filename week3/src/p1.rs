#![allow(unused)]

fn main() {
	// FIRST APPROACH
	// convert each row in input matrix to a bitstring
	// recursively sequential bitwise and
	// find the longest contiguous 1s; update L increment W
	//
	// output L*W
	//
	// SECOND APPROACH
	// take the sum of all columns
	// find consecutive items greater than x and decrement x
	// the first time get two or more consecutive items (y) greater x return x*y
	//
	let input: Vec<Vec<u8>> = vec![
		vec![1, 0, 1, 1, 0],
		vec![1, 0, 1, 1, 1],
		vec![1, 1, 1, 1, 1],
		vec![1, 0, 1, 1, 0],
	];
	/*
	 *         ⬐ count starts here
	 *  [1]   [2]        [3]        [4]
	 *
	 * 10100 10000(0*2) 10000(0*3) 10000(0*4)
	 * 10111 10111(3*2) 10110(2*3)
	 * 11111 10110(2*2)
	 * 10110
	 */

	println!("Max area: \x1b[33m{}", find_longest(input, 0, 0).1);
}

fn find_longest(input: Vec<Vec<u8>>, mut count: u8, mut max_area: u8) -> (u8, u8) {
	// escape
	if input.len() == 1 {
		return (count, max_area);
	}

	let mut intermediate: Vec<Vec<u8>> = vec![];

	for i in 0..(input.len() - 1) {
		let y = input[i]
			.iter()
			.zip(input[i + 1].iter())
			.map(|(x, y)| x & y)
			.collect::<Vec<u8>>();

		let this_area = contiguous_ones(&y) * (count + 2);

		if this_area > max_area {
			max_area = this_area;
		}

		intermediate.push(y);
	}

	return find_longest(intermediate, count + 1, max_area);
}

fn contiguous_ones(x: &[u8]) -> u8 {
	// convert Vec<u8> to Vec<String>
	let bitstring = x
		.iter()
		.map(|x| x.to_string())
		.collect::<Vec<String>>()
		.join("");

	let mut y = bitstring.split('0').collect::<Vec<&str>>();

	y.sort(); // O(N·LogN)

	let z = y.last().unwrap().len() as u8;

	if z > 1 {
		z
	} else {
		0
	}
}

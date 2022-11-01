fn main() {
	/*
	 * skip the first round, instead
	 * initialise an array[100] with `false`. In the prelude, bool is of size 1 and alignment 1.
	 * And internally stored as 0x00 and 0x01.
	 * start from round 2
	 * flip the ith element
	 * at the end sum should give the number of open doors
	 */

	let mut doors = [false; 100];

	(1..100).for_each(|i| {
		// starting at the ith item (starting at 0 caused false positives)
		doors[i..100].iter_mut().step_by(i).for_each(|x| *x = !*x);
	});

	let open_doors: i32 = doors.iter().map(|x| *x as i32).sum();

	println!("{open_doors}");
}

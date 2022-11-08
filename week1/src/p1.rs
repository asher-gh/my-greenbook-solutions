fn main() {
	/*
	 * skip the first round, instead
	 * initialise an array[100] with `false`. In the prelude, bool is of size 1 and alignment 1.
	 * And internally stored as 0x00 and 0x01.
	 * start from round 2
	 * flip the ith element
	 * at the end sum should give the number of open doors
	 */

	// round 0

	const M: usize = 100; // rounds
	const N: usize = 100; // lockers

	let mut door_arr = [false; N];

	// starting with round 1 and every ith item
	(1..M).for_each(|i| door_arr.iter_mut().step_by(i).for_each(|x| *x = !*x));

	let open_doors: usize = door_arr.iter().filter(|&&x| x == true).count();

	println!("{open_doors}");
}

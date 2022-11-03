/*
* Task
* 1 in 1000 bottles is poisoned.
* 10 strips available to detect poison.
* One test per day and 7 days for result.
*
* How would you figure out the poisoned bottle in as few days as possible?
*/
#![allow(unused)]

use rand::Rng;

struct Node<'a> {
	bottles: &'a [bool],
	left: &'a Node<'a>,
	right: &'a Node<'a>,
}

impl<'a> Node<'a> {
	fn is_poisoned(self) -> bool {
		// testing if the bottles in certain range are poisoned
		return self.bottles.iter().fold(false, |a: bool, i| a | *i);
	}
}

struct BinTree<'a> {
	root: &'a Node<'a>,
}

impl<'a> BinTree<'a> {}

fn main() {
	// 1000 bottles all set to false
	let mut bottles = [false; 1000];

	// random bottle to poison
	let chosen = rand::thread_rng().gen_range(0..bottles.len());
	bottles[chosen] = true;

	println!("poisoned bottle: {chosen}");
}

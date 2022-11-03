/*
* Task
* 1 in 1000 bottles is poisoned.
* 10 strips available to detect poison.
* One test per day and 7 days to get the result.
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
	fn contains_poisoned_bottle(&self) -> bool {
		return self.bottles.iter().fold(false, |a: bool, i| a | *i);
	}
}

struct BinTree<'a> {
	root: &'a Node<'a>,
}

impl<'a> BinTree<'a> {
	// populate the tree, such that each level correspond to
	// a strip
	fn prepare_strips(&self) {
		todo!();
	}
}

fn main() {
	// 1000 bottles
	let mut bottles = [false; 1000];

	// poison a random bottle
	let chosen = rand::thread_rng().gen_range(0..bottles.len());
	bottles[chosen] = true;

	dbg!(chosen); // stderr for debugging
}

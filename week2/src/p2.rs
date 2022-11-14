use std::{
	collections::HashMap,
	io::{self, Read},
	process,
};

use rand::{rngs::ThreadRng, Rng};

// enum Door {}

fn main() {
	let mut rng = rand::thread_rng();

	let mut tally: i32 = 0;

	// choose
	println!("1. Hold on to the door you selected originally\n2. Switch and select the other door");

	let mut reader = io::stdin();
	let mut buffer = [0; 1];
	let mut choice = 0_usize;

	reader.read(&mut buffer).unwrap();

	let picked = (buffer[0] as char).to_digit(10).unwrap_or(0);

	match picked {
		0 => {
			println!("enter 1 or 2");
			process::exit(1);
		}
		// 0 -> hold
		1 => choice = 0,
		2 => choice = 1,
		_ => {}
	}

	for i in 0..100000 {
		let result = doors_init(&mut rng)[choice];
		tally += if result { 1 } else { -1 };
	}

	print!(
		"{} {} times",
		if tally < 0 { "lost" } else { "won" },
		tally.abs()
	);
}

fn doors_init(rng: &mut ThreadRng) -> [bool; 2] {
	// randomly choose true of false for hold
	let hold: bool = rng.gen();
	[hold, !hold]
}

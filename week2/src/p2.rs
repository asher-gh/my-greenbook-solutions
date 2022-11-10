use rand::Rng;

fn main() {
	let mut rng = rand::thread_rng();

	for i in 0..10 {
		let x: u32 = rng.gen();
		println!("{x}");
	}
}

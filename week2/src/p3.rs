#![allow(unused)]

fn main() {
	let mut x: Vec<i32> = vec![4, 17, 5, 6, 12, 1, 9];

	println!("unsorted: {:?}", x);

	let (mut len, mut i) = (x.len(), 0);

	while i < len {
		let first_el = i == 0 && x[i] > x[i + 1];
		let last_el = i == len - 1 && x[i] < x[i - 1];

		if i == 0 || i == len - 1 {
			if first_el || last_el {
				x.remove(i);
				i-=1;
			}
		} else if (x[i] < x[i - 1] || x[i] > x[i + 1]) {
			println!("removing {}", x[i]);
			x.remove(i);
			i-=1;
		} 
			i+=1;
		

		len = x.len();
	}

	println!("\n{:?}", x);
}

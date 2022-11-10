mod p1;
use p1::substring;
use std::collections::HashMap;

fn main() {
	let s1 = "abCd";
	let s2 = "somethingabCd";

	for s in get_substrings(s1) {
		substring::find_substrings(s, s2);
	}
}

fn get_substrings(s: &str) -> Vec<&str> {
	// find all substrings of s1
	// [s,b,C,d]
	//  ↑
	//  p
	//
	//  [s, sb, sbc, sbcd, b, bc, bcd, c, cd, d]
	//
	let mut substr: Vec<&str> = vec![s];

	for i in 0..s.len() {
		for j in i..s.len() {
			substr.push(s.get(i..j + 1).unwrap());
		}
	}

	substr
}

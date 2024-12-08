use std::io;
use std::collections::HashMap;

pub fn main() {
	let mut d1:Vec<i32> = Vec::new();
	let mut d2:Vec<i32> = Vec::new();
	let mut ct:HashMap<i32,i32> = HashMap::new();
	for i in io::stdin().lines() {
		let a:Vec<_> = i.unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
		d2.push(a[1]);
		d1.push(a[0]);
		*ct.entry(a[1]).or_default() += 1;
	}

	d1.sort();
	d2.sort();
	let mut z = 0;
	let mut z2 = 0;
	for i in 0..d1.len() {
		z += (d1[i] - d2[i]).abs();
		z2 += d1[i] * *ct.entry(d1[i]).or_default();
	}
	println!("{z}");
	println!("{z2}");
}

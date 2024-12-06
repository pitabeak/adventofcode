use std::io;
use itertools::Itertools;
use std::collections::HashMap;

pub fn main() {
	let mut d1 = Vec::new();
	let mut d2 = Vec::new();
	let mut ct:HashMap<i32,i32> = HashMap::new();
	for i in io::stdin().lines() {
		let i = i.unwrap();
		let (x,y) = i.split_whitespace()
			.map(|x| x.parse::<i32>().unwrap())
			.collect_tuple::<(_,_)>().unwrap();
		d1.push(x);
		d2.push(y);
		*ct.entry(y).or_default() += 1;
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

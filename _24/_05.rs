use std::io;
use std::collections::HashSet;

pub fn main() {
	let mut it = io::stdin().lines().map(|x| String::from(x.unwrap().trim()));
	let mut d1:HashSet<(i8,i8)> = HashSet::new();
	for i in it.by_ref().take_while(|s| s.len() > 0) {
		let a = i.split_once('|').unwrap();
		d1.insert((a.0.parse().unwrap(),a.1.parse().unwrap()));
	}
	let ngt = |x,y| !d1.contains(&(y,x));
	let mut z = 0;
	let mut z2 = 0;
	for i in it {
		let mut a:Vec<i8> = i.split(',').map(|s| s.parse().unwrap()).collect();
		if (1..a.len()).all(|i| (0..i).all(|j| ngt(a[j],a[i]))) {
			z += a[a.len()/2] as i32;
		} else {
			for i in 0..a.len()-1 {
				let b = &a[i..];
				let j = b.iter().position(|&x| b.iter().all(|&y| ngt(x,y))).unwrap();
				if j > 0 { a.swap(i,i+j); }
			}
			z2 += a[a.len()/2] as i32;
		}
	}
	println!("{z}");
	println!("{z2}");
}

use std::io::BufRead;
use std::collections::HashMap;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut da:Vec<Vec<usize>> = Vec::new();
	let mut i1 = Vec::new();
	let mut n = 0;
	for (y,i) in f.lines().enumerate() {
		let i = i.unwrap();
		let i = i.as_bytes();
		let mut rw = Vec::new();
		for x in 0..i.len() {
			let b = i[x];
			if y > 0 && b == i1[x] {
				if x > 0 && b == i[x-1] {
					let p = da[y-1][x];
					let q = rw[x-1];
					for r in rw.iter_mut() {
						if *r == q { *r = p; }
					}
					for s in da.iter_mut() {
						for r in s.iter_mut() {
							if *r == q { *r = p; }
						}
					}
				}
				rw.push(da[y-1][x]);
			} else if x > 0 && b == i[x-1] {
				rw.push(rw[x-1]);
			} else {
				rw.push(n);
				n += 1;
			}
		}
		if y == 0 { i1.resize(i.len(),0); }
		i1.copy_from_slice(i);
		da.push(rw);
	}
	let mx = da[0].len()-1;
	let my = da.len()-1;
	let mut sd = vec![0;n];
	let mut pm = vec![0;n];
	let mut ar = vec![0;n];
	for y in 0..=my {
		let mut ra = false;
		let mut rb = false;
		for x in 0..=mx {
			let i = da[y][x];
			ar[i] += 1;
			if y > 0 && i == da[y-1][x] {
				ra = false;
			} else {
				if !(ra && i == da[y][x-1]) {
					sd[i] += 1;
				}
				pm[i] += 1;
				ra = true;
			}
			if y < my && i == da[y+1][x] {
				rb = false;
			} else {
				if !(rb && i == da[y][x-1]) {
					sd[i] += 1;
				}
				pm[i] += 1;
				rb = true;
			}
		}
	}
	for x in 0..=mx {
		let mut ra = false;
		let mut rb = false;
		for y in 0..=my {
			let i = da[y][x];
			if x > 0 && i == da[y][x-1] {
				ra = false;
			} else {
				if !(ra && i == da[y-1][x]) {
					sd[i] += 1;
				}
				pm[i] += 1;
				ra = true;
			}
			if x < mx && i == da[y][x+1] {
				rb = false;
			} else {
				if !(rb && i == da[y-1][x]) {
					sd[i] += 1;
				}
				pm[i] += 1;
				rb = true;
			}
		}
	}
	let z:usize = (0..ar.len()).map(|i| ar[i]*pm[i]).sum();
	let z2:usize = (0..ar.len()).map(|i| ar[i]*sd[i]).sum();
	(z.to_string(),z2.to_string())
}

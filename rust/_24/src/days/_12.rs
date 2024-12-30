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
	let mut ar:HashMap<usize,usize> = HashMap::new();
	let mut hz:HashMap<usize,Vec<(usize,usize)>> = HashMap::new();
	let mut vt:HashMap<usize,Vec<(usize,usize)>> = HashMap::new();
	for y in 0..=my {
		for x in 0..=mx {
			let p = da[y][x];
			*ar.entry(p).or_default() += 1;
			if !(y > 0 && da[y-1][x] == p) {
				hz.entry(p).or_default().push((x,y));
			}
			if !(y < my && da[y+1][x] == p) {
				hz.entry(p).or_default().push((x,y+1));
			}
			if !(x > 0 && da[y][x-1] == p) {
				vt.entry(p).or_default().push((x,y));
			}
			if !(x < mx && da[y][x+1] == p) {
				vt.entry(p).or_default().push((x+1,y));
			}
		}
	}
	let z:usize = ar.iter().map(|(i,n)| n*(hz[i].len()+vt[i].len())).sum();
	let mut sd:HashMap<usize,usize> = HashMap::new();
	for y in 0..=my {
		let mut a = false;
		let mut b = false;
		for x in 0..=mx {
			if y==0 || da[y][x]!=da[y-1][x] {
				if !a || da[y][x]!=da[y][x-1] {
					*sd.entry(da[y][x]).or_default() += 1;
					a = true;
				}
			} else { a = false; }
			if y==my || da[y][x]!=da[y+1][x] {
				if !b || da[y][x]!=da[y][x-1] {
					*sd.entry(da[y][x]).or_default() += 1;
					b = true;
				}
			} else { b = false; }
		}
	}
	for x in 0..=mx {
		let mut a = false;
		let mut b = false;
		for y in 0..=my {
			if x==0 || da[y][x]!=da[y][x-1] {
				if !a || da[y][x]!=da[y-1][x] {
					*sd.entry(da[y][x]).or_default() += 1;
					a = true;
				}
			} else { a = false; }
			if x==mx || da[y][x]!=da[y][x+1] {
				if !b || da[y][x]!=da[y-1][x] {
					*sd.entry(da[y][x]).or_default() += 1;
					b = true;
				}
			} else { b = false; }
		}
	}
	let z2:usize = sd.iter().map(|(i,n)| n*ar[i]).sum();
	(z.to_string(),z2.to_string())
}

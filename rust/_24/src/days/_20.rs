use std::io::BufRead;
use std::io;
use std::collections::{HashMap,BinaryHeap};
use std::cmp::Reverse;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let da = io::read_to_string(f).unwrap();
	let da:Vec<_> = da.lines().map(|s| s.as_bytes()).collect();
	let wd = da[0].len();
	let ht = da.len();
	let st = (0..wd*ht).map(|i| (i%wd,i/wd)).filter(|&(x,y)| da[y][x]==b'S').next().unwrap();
	let ed = (0..wd*ht).map(|i| (i%wd,i/wd)).filter(|&(x,y)| da[y][x]==b'E').next().unwrap();
	let is_open = |(x,y):(usize,usize)| da[y][x] != b'#';
	let mut ds:HashMap<(usize,usize),usize> = HashMap::new();
	let mut qu = BinaryHeap::new();
	ds.insert(st,0);
	qu.push(Reverse((0,st)));
	while let Some(Reverse((n,p))) = qu.pop() {
		let q = (p.0-1,p.1);
		if !ds.contains_key(&q) && is_open(q) {
			ds.insert(q,n+1);
			qu.push(Reverse((n+1,q)));
		}
		let q = (p.0+1,p.1);
		if !ds.contains_key(&q) && is_open(q) {
			ds.insert(q,n+1);
			qu.push(Reverse((n+1,q)));
		}
		let q = (p.0,p.1-1);
		if !ds.contains_key(&q) && is_open(q) {
			ds.insert(q,n+1);
			qu.push(Reverse((n+1,q)));
		}
		let q = (p.0,p.1+1);
		if !ds.contains_key(&q) && is_open(q) {
			ds.insert(q,n+1);
			qu.push(Reverse((n+1,q)));
		}
	}
	let mut z = 0;
	//let mut ct:HashMap<usize,usize> = HashMap::new();
	for y in 1..ht-1 {
		for x in 1..wd-1 {
			if !is_open((x,y)) {
				if is_open((x-1,y)) && is_open((x+1,y)) {
					let d = ds[&(x-1,y)].abs_diff(ds[&(x+1,y)]) - 2;
					if d >= 100 { z += 1; }
	//				*ct.entry(d).or_default() += 1;
				}
				if is_open((x,y-1)) && is_open((x,y+1)) {
					let d = ds[&(x,y-1)].abs_diff(ds[&(x,y+1)]) - 2;
					if d >= 100 { z += 1; }
	//				*ct.entry(d).or_default() += 1;
				}
			}
		}
	}
	//println!("{ct:?}");
	(z.to_string(),0.to_string())
}

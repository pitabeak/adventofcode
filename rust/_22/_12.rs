use std::io::BufRead;
use std::io;
use std::collections::{HashMap,BinaryHeap};
use std::collections::hash_map::Entry;
use std::cmp::Reverse;

fn find<F:Fn((usize,usize))->u8>(wd:usize,ht:usize,st:(usize,usize),ed:(usize,usize),level:F) -> Option<usize> {
	let mut ds = HashMap::new();
	let mut qu = BinaryHeap::new();
	ds.insert(st,0);
	qu.push(Reverse((0,st)));
	while let Some(Reverse((n,q))) = qu.pop() {
		let a = level(q);
		if q.0 > 0 {
			let p = (q.0-1,q.1);
			if level(p) <= level(q)+1 {
				if let Entry::Vacant(e) = ds.entry(p) {
					e.insert(n+1);
					qu.push(Reverse((n+1,p)));
				}
			}
		}
		if q.1 > 0 {
			let p = (q.0,q.1-1);
			if level(p) <= level(q)+1 {
				if let Entry::Vacant(e) = ds.entry(p) {
					e.insert(n+1);
					qu.push(Reverse((n+1,p)));
				}
			}
		}
		if q.0 < wd-1 {
			let p = (q.0+1,q.1);
			if level(p) <= level(q)+1 {
				if let Entry::Vacant(e) = ds.entry(p) {
					e.insert(n+1);
					qu.push(Reverse((n+1,p)));
				}
			}
		}
		if q.1 < ht-1 {
			let p = (q.0,q.1+1);
			if level(p) <= level(q)+1 {
				if let Entry::Vacant(e) = ds.entry(p) {
					e.insert(n+1);
					qu.push(Reverse((n+1,p)));
				}
			}
		}
	}
	ds.get(&ed).map(|&x| x)
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let da = io::read_to_string(f).unwrap();
	let da:Vec<_> = da.lines().map(|s| s.as_bytes()).collect();
	let wd = da[0].len();
	let ht = da.len();
	let level = |p:(usize,usize)| {
			let a = da[p.1][p.0];
			match a {
				b'S' => b'a',
				b'E' => b'z',
				_ => a
			}
		};
	let st = (0..wd*ht).map(|i| (i%wd,i/wd)).filter_map(|p| (da[p.1][p.0]==b'S').then_some(p)).next().unwrap();
	let ed = (0..wd*ht).map(|i| (i%wd,i/wd)).filter_map(|p| (da[p.1][p.0]==b'E').then_some(p)).next().unwrap();
	let z = find(wd,ht,st,ed,level).unwrap();
	let z2 = (0..wd*ht).map(|i| (i%wd,i/wd))
		.filter(|&p| level(p)==b'a').filter_map(|p| find(wd,ht,p,ed,level)).min().unwrap();
	(z.to_string(),z2.to_string())
}

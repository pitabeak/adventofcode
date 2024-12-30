use std::io::BufRead;
use std::io;
use std::collections::{HashMap,BinaryHeap};
use std::cmp::Reverse;

fn find(cl:usize,wd:usize,ht:usize,ds:&HashMap<(usize,usize),usize>) -> usize {
	let mut z = 0;
	for y in 1..ht-1 {
		for x in 1..wd-1 {
			if let Some(&a) = ds.get(&(x,y)) {
				for y1 in y-cl.min(y)..=y+cl.min(ht-1-y) {
					let r = cl - y1.abs_diff(y);
					for x1 in x-r.min(x)..=x+r.min(wd-1-x) {
						let m = y1.abs_diff(y) + x1.abs_diff(x);
						if m%2 == 0 || true {
							if let Some(&b) = ds.get(&(x1,y1)) {
								if let Some(d) = b.checked_sub(a + y1.abs_diff(y) + x1.abs_diff(x)) {
									if d >= 100 { z += 1; }
								}
							}
						}
					}
				}
			}
		}
	}
	z
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let da = io::read_to_string(f).unwrap();
	let da:Vec<_> = da.lines().map(|s| s.as_bytes()).collect();
	let wd = da[0].len();
	let ht = da.len();
	let st = (0..wd*ht).map(|i| (i%wd,i/wd)).filter(|&(x,y)| da[y][x]==b'S').next().unwrap();
	let ed = (0..wd*ht).map(|i| (i%wd,i/wd)).filter(|&(x,y)| da[y][x]==b'E').next().unwrap();
	let is_open = |(x,y):(usize,usize)| da[y][x] != b'#';
	let nb = [(-1,0),(1,0),(0,-1),(0,1)];
	let mut ds:HashMap<(usize,usize),usize> = HashMap::new();
	let mut qu = BinaryHeap::new();
	ds.insert(st,0);
	qu.push(Reverse((0,st)));
	while let Some(Reverse((n,p))) = qu.pop() {
		for q in nb.iter().map(|(dx,dy)| ((p.0 as isize + dx) as usize,(p.1 as isize + dy) as usize)) {
			if !ds.contains_key(&q) && is_open(q) {
				ds.insert(q,n+1);
				qu.push(Reverse((n+1,q)));
			}
		}
	}
	let mut z = find(2,wd,ht,&ds);
	let mut z2 = find(20,wd,ht,&ds);
	(z.to_string(),z2.to_string())
}

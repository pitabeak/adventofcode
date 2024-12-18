use std::io::BufRead;
use std::collections::{HashSet,HashMap,BinaryHeap};
use std::cmp::Reverse;
use std::collections::hash_map::Entry;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut da = HashSet::new();
	let wd = 70;
	let fr = 1024;
	let mut z = 0;
	let mut z2 = String::new();
	for (j,i) in f.lines().enumerate() {
		let i = i.unwrap();
		let (x,y) = i.split_once(',').unwrap();
		let x:usize = x.parse().unwrap();
		let y:usize = y.parse().unwrap();
		da.insert((x,y));
		if j < fr { continue; }
		let mut ds = HashMap::new();
		let mut qu = BinaryHeap::new();
		qu.push(Reverse((0,(0,0))));
		ds.insert((0,0),0);
		while let Some(Reverse((n,(x,y)))) = qu.pop() {
			if x > 0 {
				let p = (x-1,y);
				if !da.contains(&p) {
					if let Entry::Vacant(e) = ds.entry(p) {
						e.insert(n+1);
						qu.push(Reverse((n+1,p)));
					}
				}
			}
			if y > 0 {
				let p = (x,y-1);
				if !da.contains(&p) {
					if let Entry::Vacant(e) = ds.entry(p) {
						e.insert(n+1);
						qu.push(Reverse((n+1,p)));
					}
				}
			}
			if x < wd {
				let p = (x+1,y);
				if !da.contains(&p) {
					if let Entry::Vacant(e) = ds.entry(p) {
						e.insert(n+1);
						qu.push(Reverse((n+1,p)));
					}
				}
			}
			if y < wd {
				let p = (x,y+1);
				if !da.contains(&p) {
					if let Entry::Vacant(e) = ds.entry(p) {
						e.insert(n+1);
						qu.push(Reverse((n+1,p)));
					}
				}
			}
		}
		let np = ds.get(&(wd,wd));
		if j == fr { z = *np.unwrap(); }
		else if np.is_none() { z2 = format!("{x},{y}"); break; }
	}
	(z.to_string(),z2)
}

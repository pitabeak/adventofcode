use std::io::BufRead;
use std::collections::{HashMap,HashSet};

const M:i64 = 16777216;

fn nxt(mut x:i64) -> i64 {
	x = (x*64i64^x)%M;
	x = (x/32i64^x)%M;
	x = (x*2048i64^x)%M;
	x
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut z = 0;
	let mut bm:HashMap<(i8,i8,i8,i8),usize> = HashMap::new();
	for s in f.lines() {
		let mut x = s.unwrap().parse().unwrap();
		let mut t = (0,0,0,0);
		let mut d = (x%10) as i8;
		let mut ts = HashSet::new();
		for i in 0..2000 {
			x = nxt(x);
			let d1 = (x%10) as i8;
			t = (t.1,t.2,t.3,d1-d);
			d = d1;
			if i >= 3 && !ts.contains(&t) {
				ts.insert(t);
				*bm.entry(t).or_default() += d as usize;
			}
		}
		z += x;
	}
	let z2 = bm.values().max().unwrap();
	(z.to_string(),z2.to_string())
}

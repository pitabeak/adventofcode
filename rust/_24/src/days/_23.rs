use std::io::BufRead;
use std::io;
use std::collections::{HashMap,HashSet};
use itertools::Itertools;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut pr:HashMap<(u8,u8),HashSet<(u8,u8)>> = HashMap::new();
	for s in f.lines() {
		let s = s.unwrap();
		let s = s.as_bytes();
		let mut x = (s[0],s[1]);
		let mut y = (s[3],s[4]);
		if y < x { (x,y) = (y,x); }
		pr.entry(x).or_default().insert(y);
	}
	let mut z = 0;
	let mut z2 = String::new();
	let mut sl = 4;
	for (x,v) in &pr {
		for y in v {
			if let Some(w) = pr.get(&y) {
				for q in w {
					if (x.0==b't' || y.0==b't' || q.0==b't') && pr[&x].contains(&q) {
						z += 1;
					}
				}
			}
		}
		let mut v:Vec<_> = v.iter().collect();
		v.sort();
		let mut g = true;
		while g {
			g = false;
			for w in (0..v.len()).combinations(sl-1) {
				if (0..w.len()-1).all(|i| pr.get(v[w[i]]).is_some_and(|s| (i+1..w.len()).
					all(|j| s.contains(v[w[j]])))) {
					g = true;
					sl += 1;
					let mut a:Vec<_> = v.iter().map(|&&(a,b)| String::from_utf8_lossy(&[a,b]).to_string()).collect();
					a.insert(0,String::from_utf8_lossy(&[x.0,x.1]).to_string());
					z2 = a.join(",");
				}
			}
		}
	}
	println!("{}",pr.iter().map(|(_,v)| v.len()).max().unwrap());
	(z.to_string(),z2.to_string())
}

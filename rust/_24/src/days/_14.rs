use std::io::BufRead;
use regex::Regex;
use std::collections::HashSet;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let re = Regex::new(r"-?\d+").unwrap();
	let wd = 101;
	let ht = 103;
	let mut po = Vec::new();
	let mut sp = Vec::new();
	for i in f.lines() {
		let a:Vec<i32> = re.find_iter(&i.unwrap()).map(|m| m.as_str().parse().unwrap()).collect();
		po.push([a[0],a[1]]);
		sp.push([a[2],a[3]]);
	}
	let mut qu = [0;4];
	let mut m = 0;
	let mut z2 = 0;
	for j in 1.. {
		for i in 0..po.len() {
			po[i][0] = (po[i][0] + wd + sp[i][0])%wd;
			po[i][1] = (po[i][1] + ht + sp[i][1])%ht;
		}
		if j == 100 {
			for &[x,y] in &po {
				if !(x == wd/2 || y == ht/2) {
					let j = if x<wd/2 { if y<ht/2 {0} else {1} }
					else { if y<ht/2 {2} else {3} };
					qu[j] += 1;
				}
			}
		}
		let mp:HashSet<&[i32;2]> = HashSet::from_iter(po.iter());
		if mp.len() == po.len() {
			m += 1;
			if m==2 { z2 = j; break; }
		}
	}
	let z:i32 = qu.iter().product();
	(z.to_string(),z2.to_string())
}

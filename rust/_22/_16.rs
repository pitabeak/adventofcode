use std::io::BufRead;
use std::collections::{HashMap,BinaryHeap};
use std::cmp::Reverse;
use regex::Regex;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
	let mut nm = HashMap::new();
	let mut da = Vec::new();
	for s in f.lines() {
		let s = s.unwrap();
		let c = re.captures(&s).unwrap();
		let t = &c[1].as_bytes();
		let l = nm.len();
		let i = *nm.entry([t[0],t[1]]).or_insert(l);
		while da.len()<=i { da.push(Vec::new()); }
		let v = &mut da[i];
		let n:usize = c[2].parse().unwrap();
		v.push(n);
		let t = &c[3].as_bytes();
		for j in (0..t.len()).step_by(4) {
			let l = nm.len();
			let i = *nm.entry([t[j],t[j+1]]).or_insert(l);
			v.push(i);
		}
	}
	let st = nm[b"AA"];
	let mut wk = Vec::new();
	for i in 0..da.len() {
		let mut qu = BinaryHeap::new();
		qu.push(Reverse((0,i)));
		let mut v = vec![usize::MAX;da.len()];
		while let Some(Reverse((d,j))) = qu.pop() {
			v[j] = d;
			for &k in da[j].iter().skip(1) {
				if v[k]==usize::MAX {
					qu.push(Reverse((d+1,k)));
				}
			}
		}
		wk.push(v);
	}
	let nz:Vec<_> = (0..da.len()).filter(|&i| da[i][0]!=0).collect();
	let nk:Vec<_> = nz.iter().
		map(|&y| { let mut w:Vec<_> = nz.iter().map(|&x| wk[y][x]).collect(); w.insert(0,da[y][0]); w }).
		collect();
	let sk:Vec<_> = nz.iter().map(|&x| wk[st][x]).collect();
	println!("{sk:?}");
	println!("{nk:?}");
	(0.to_string(),0.to_string())
}

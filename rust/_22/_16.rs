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
		map(|&y| { let mut w:Vec<_> = nz.iter().map(|&x| wk[y][x]+1).collect(); w.insert(0,da[y][0]); w }).
		collect();
	let sk:Vec<_> = nz.iter().map(|&x| wk[st][x]+1).collect();
	
	let mut z = 0;
	const M:usize = 30;
	let mut qu = Vec::new();
	for i in 0..sk.len() {
		let m = sk[i];
		let p = (M-m)*nk[i][0];
		qu.push((p,m,i,1<<i));
	}
	while let Some((p,m,i,b)) = qu.pop() {
		z = z.max(p);
		for j in 0..sk.len() {
			if b&(1<<j)==0 && m+nk[i][j+1]<M {
				let n = m+nk[i][j+1];
				let q = p+(M-n)*nk[j][0];
				qu.push((q,n,j,b|(1<<j)));
			}
		}
	}

	let mut z2 = 0;
	const M2:usize = 26;
	let mut qu = Vec::new();
	for i in (0..sk.len()).step_by(2) {
		let m = sk[i];
		let p = (M2-m)*nk[i][0];
		qu.push((p,m,i,1<<i,true));
	}
	while let Some((p,m,i,b,u)) = qu.pop() {
		z2 = z2.max(p);
		for j in 0..sk.len() {
			if b&(1<<j)==0 && m+nk[i][j+1]<M2 {
				let n = m+nk[i][j+1];
				let q = p+(M2-n)*nk[j][0];
				qu.push((q,n,j,b|(1<<j),u));
			}
		}
		if u {
			for j in (1..sk.len()).step_by(2) {
				if b&(1<<j)==0 {
					let n = sk[j];
					let q = p+(M2-n)*nk[j][0];
					qu.push((q,n,j,b|(1<<j),false));
				}
			}
		}
	}
	(z.to_string(),z2.to_string())
}

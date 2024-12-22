use std::io::BufRead;
use regex::Regex;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let re = Regex::new(r"\d+").unwrap();
	let mut it = f.lines().map(|s| s.unwrap());
	let mut a = Vec::new();
	loop {
		let s = it.next().unwrap();
		let s = s.as_bytes();
		if !(s[1] == b' ' || s[1] >= b'A') {
			let i = s.len()/4;
			while a.len() <= i { a.push(Vec::new()); }
			break;
		}
		for j in (1..s.len()).step_by(4) {
			if s[j] != b' ' {
				let i = j/4;
				while a.len() <= i { a.push(Vec::new()); }
				a[i].insert(0,s[j]);
			}
		}
	}
	let mut a1 = a.to_vec();
	it.next();
	let mut b = Vec::with_capacity(3);
	for s in it {
		b.clear();
		b.extend(re.find_iter(&s).map(|m| m.as_str().parse::<usize>().unwrap()));
		let k = a[b[1]-1].len()-b[0];
		for i in 0..b[0] {
			let x = a[b[1]-1].pop().unwrap();
			a[b[2]-1].push(x);
			let x = a1[b[1]-1].remove(k);
			a1[b[2]-1].push(x);
		}
	}
	let z:String = a.iter().map(|v| v[v.len()-1] as char).collect();
	let z2:String = a1.iter().map(|v| v[v.len()-1] as char).collect();
	(z,z2)
}

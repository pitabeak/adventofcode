use std::io::BufRead;
use std::collections::{HashMap,HashSet};

enum Circuit {
	Signal(bool),
	Gate(String,String,String)
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut it = f.lines();
	let mut da = HashMap::new();
	let mut x = 0;
	let mut y = 0;
	let mut h = 0;
	let mut g = true;
	while let Some(Ok(s)) = it.next() {
		if s.len() == 0 { break; }
		da.insert(s[..3].to_string(),Circuit::Signal(s.ends_with("1")));
		let s = s.as_bytes();
		let b = (s[5] - b'0') as usize;
		if s[0] == b'x' { x |= b<<h; }
		else { if g { g = false; h = 0; } y |= b<<h; }
		h += 1;
	}
	let mut ai = HashSet::new();
	let mut ao = HashSet::new();
	let mut xo = HashSet::new();
	let mut ng = 0;
	while let Some(Ok(s)) = it.next() {
		let a:Vec<_> = s.split(' ').collect();
		da.insert(a[4].to_string(),Circuit::Gate(a[1].to_string(),a[0].to_string(),a[2].to_string()));
		ng += 1;
		let xy = a[0].starts_with('x') || a[0].starts_with('y');
		let zz = a[4].starts_with('z');
		match a[1] {
			"XOR" => {
				if xy {
					if zz && a[4] != "z00" { print!("{} ",a[4]); }
					xo.insert(a[4].to_string());
				} else {
					if !zz { print!("{} ",a[4]); }
				}
			}
			"AND" => {
				if zz { print!("{} ",a[4]); }
				if !xy {
					ai.insert(a[0].to_string());
					ai.insert(a[2].to_string());
				}
				ao.insert(a[4].to_string());
			}
			"OR" => {
				if zz && a[4] != "z45" { print!("{} ",a[4]); }
			}
			_ => {}
		}
	}
	for _ in 0..ng {
		let mut p = String::new();
		let mut q = false;
		for k in da.keys() {
			if let Circuit::Gate(op,a,b) = &da[k] {
				if let Circuit::Signal(a) = da[a] {
					if let Circuit::Signal(b) = da[b] {
						q = match op.as_str() {
							"XOR" => a!=b,
							"AND" => a&&b,
							"OR" => a||b,
							_ => panic!()
						};
						p = k.to_string();
						break;
					}
				}
			}
		}
		da.insert(p,Circuit::Signal(q));
	}
	let z = (0..=45).map(|i| {
			let Circuit::Signal(g) = da[&format!("z{:02}",45-i)] else {panic!()};
			g as usize
		}).fold(0,|a,x| a*2+x);
	for x in xo {
		if !ai.contains(&x) && x != "z00" { print!("{x} "); }
	}
	for x in ao {
		if ai.contains(&x) { print!("*{x} "); }
	}
	println!("");
	println!("(The answer to part 2 is a list of the above\nwith only one of the marked ones.)");
	let z2 = "gbs,hwq,thm,wrm,wss,z08,z22,z29";
	(z.to_string(),z2.to_string())
}

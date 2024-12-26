use std::io::BufRead;
use std::collections::HashSet;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut it = f.lines();
	let mut x = 0;
	let mut y = 0;
	let mut h = 0;
	let mut g = true;
	while let Some(Ok(s)) = it.next() {
		if s.len() == 0 { break; }
		let s = s.as_bytes();
		let b = (s[5] - b'0') as usize;
		if s[0] == b'x' { x |= b<<h; }
		else { if g { g = false; h = 0; } y |= b<<h; }
		h += 1;
	}
	let mut oi = HashSet::new();
	let mut oo = HashSet::new();
	let mut ai = HashSet::new();
	let mut ao = HashSet::new();
	while let Some(Ok(s)) = it.next() {
		let a:Vec<_> = s.split(' ').collect();
		let xy = a[0].starts_with('x') || a[0].starts_with('y');
		let zz = a[4].starts_with('z');
		match a[1] {
			"XOR" => {
				if xy {
					if zz { println!("{s}"); }
				} else {
					if !zz { println!("{s}"); }
				}
			}
			"AND" => {
				if zz { println!("{s}"); }
				if !xy {
					ai.insert(a[0].to_string());
					ai.insert(a[2].to_string());
				}
				ao.insert(a[4].to_string());
			}
			"OR" => {
				if zz { println!("{s}"); }
				oi.insert(a[0].to_string());
				oi.insert(a[2].to_string());
				oo.insert(a[4].to_string());
			}
			_ => {}
		}
	}
	for x in oo {
		if !ai.contains(&x) { println!("+{x}"); }
	}
	for x in ao {
		if !oi.contains(&x) { println!("-{x}"); }
	}
	(0.to_string(),0.to_string())
}

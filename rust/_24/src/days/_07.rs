use std::io::BufRead;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut z = 0;
	let mut z2 = 0;
	for i in f.lines() {
		let i = i.unwrap();
		let (a,b) = i.split_once(": ").unwrap();
		let a:i64 = a.parse().unwrap();
		let mut b:Vec<i64> = b.split_whitespace().map(|s| s.parse().unwrap()).collect();
		let c = b.remove(0);
		for i in 0..2usize.pow((b.len()) as u32) {
			let mut j = i;
			let mut x = c;
			for y in &b {
				match j%2 {
					0 => x += y,
					1 => x *= y,
					_ => todo!()
				}
				j /= 2;
			}
			if x == a { z += a; break; }
		}
		'l1:
		for i in 0..3usize.pow((b.len()) as u32) {
			let mut j = i;
			let mut x = c;
			for y in &b {
				match j%3 {
					0 => x += y,
					1 => x *= y,
					2 => {
						let mut p = x.to_string();
						let q = y.to_string();
						if p.len()+q.len() > 16 { continue 'l1; }
						p.push_str(&q);
						x = p.parse().unwrap();
					}
					_ => todo!()
				}
				j /= 3;
			}
			if x == a { z2 += a; break; }
		}
	}
	(z.to_string(),z2.to_string())
}

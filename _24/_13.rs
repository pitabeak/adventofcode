use std::io::BufRead;
use regex::Regex;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let re = Regex::new(r"(\d+)").unwrap();
	let mut it = f.lines().map(|s| s.unwrap());
	let ad = 10000000000000i64;
	let mut z = 0;
	let mut z2 = 0i64;
	loop {
		println!(".");
		let a:Vec<Vec<i64>> = (0..3).map(|_| re.find_iter(&it.next().unwrap())
			.map(|m| m.as_str().parse().unwrap()).collect()).collect();
		let ax = a[0][0];
		let ay = a[0][1];
		let bx = a[1][0];
		let by = a[1][1];
		let px = a[2][0];
		let py = a[2][1];
		let d = ax*by-ay*bx;
		let n = px*by-py*bx;
		if n%d == 0 {
			let a = n/d;
			let mut b = px - a*ax;
			if b%bx == 0 {
				b /= bx;
				z += 3*a + b;
			}
		}
		let px = px + ad;
		let py = py + ad;
		let d = ax*by-ay*bx;
		let n = px*by-py*bx;
		if n%d == 0 {
			let a = n/d;
			let mut b = px - a*ax;
			if b%bx == 0 {
				b /= bx;
				z2 += 3*a + b;
			}
		}
		if it.next().is_none() { break; }
	}
	(z.to_string(),z2.to_string())
}

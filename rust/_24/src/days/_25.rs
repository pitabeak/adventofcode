use std::io::BufRead;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut ky = Vec::new();
	let mut lk = Vec::new();
	let mut i = 0;
	let mut a = &mut Vec::new();
	for s in f.lines() {
		let b = s.unwrap();
		let b = b.as_bytes();
		if i%8 == 0 {
			let c = if b[0] == b'#' {&mut lk} else {&mut ky};
			let l = c.len();
			c.push(vec![0;5]);
			a = &mut c[l];
		}
		if i%8 != 7 {
			for j in 0..5 {
				if b[j] == b'#' {
					a[j] += 1;
				}
			}
		}
		if i%8 == 6 {
			for j in 0..5 { a[j] -= 1; }
		}
		i += 1;
	}
	let mut z = 0;
	for a in &ky {
		for b in &lk {
			if (0..5).all(|i| a[i]+b[i] <= 5) { z += 1; }
		}
	}
	(z.to_string(),0.to_string())
}

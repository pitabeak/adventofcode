use std::io::BufRead;
use std::io;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let da = io::read_to_string(f).unwrap();
	let da:Vec<_> = da.lines().map(|s| s.as_bytes()).collect();
	let wd = da[0].len();
	let ht = da.len();
	let mut z = 2*wd+2*ht-4;
	let mut z2 = 0;
	for y in 1..ht-1 {
		for x in 1..wd-1 {
			let t = da[y][x];
			let mut f = (0..x).all(|i| da[y][i] < t);
				|| (0..y).all(|i| da[i][x] < t);
				|| (1..=wd-x-1).map(|i| wd-i).all(|i| da[y][i] < t);
				|| (1..=ht-y-1).map(|i| ht-i).all(|i| da[i][x] < t);
			if f { z += 1; }
			let p = ((1..=x).take_while(|i| da[y][x-i] < t).count() + 1).min(x)
				* ((1..=y).take_while(|i| da[y-i][x] < t).count() + 1).min(y)
				* ((1..=wd-x-1).take_while(|i| da[y][x+i] < t).count() + 1).min(wd-x-1)
				* ((1..=ht-y-1).take_while(|i| da[y+i][x] < t).count() + 1).min(ht-y-1);
			z2 = z2.max(p);
		}
	}
	(z.to_string(),z2.to_string())
}

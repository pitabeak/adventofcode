use std::io::BufRead;
use regex::Regex;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let re = Regex::new(r"\d+").unwrap();
	let mut a = Vec::new();
	let mut xm = 500;
	let mut xx = 500;
	let mut yx = 0;
	for s in f.lines() {
		let s = s.unwrap();
		let b:Vec<i16> = re.find_iter(&s).map(|t| t.as_str().parse().unwrap()).collect();
		for i in (0..b.len()).step_by(2) {
			xm = xm.min(b[i]);
			xx = xx.max(b[i]);
			yx = yx.max(b[i+1]);
		}
		a.push(b.to_vec());
	}
	yx += 2;
	xm = xm.min(500-yx);
	xx = xx.max(500+yx);
	let mut mp:Vec<_> = (0..yx).map(|_| vec![false;(xx-xm+1) as usize]).collect();
	mp.push(vec![true;(xx-xm+1) as usize]);
	for b in a {
		let mut x = (b[0]-xm) as usize;
		let mut y = b[1] as usize;
		mp[y][x] = true;
		for i in (2..b.len()).step_by(2) {
			let dx = (b[i] - xm - x as i16).signum();
			let dy = (b[i+1] - y as i16).signum();
			let n = ((b[i] - xm) as usize).abs_diff(x) + (b[i+1] as usize).abs_diff(y);
			for _ in 0..n {
				x = (x as i16 + dx) as usize;
				y = (y as i16 + dy) as usize;
				mp[y][x] = true;
			}
		}
	}
	let mut x = (500-xm) as usize;
	let mut y = 0usize;
	let mut n = 1;
	let mut z = None;
	let z2;
	loop {
		if !mp[y+1][x] {
			y += 1;
		} else if !mp[y+1][x-1] {
			y += 1;
			x -= 1;
		} else if !mp[y+1][x+1] {
			y += 1;
			x += 1;
		} else {
			if z.is_none() && y==(yx-1) as usize {
				z = Some(n-1);
			}
			mp[y][x] = true;
			x = (500-xm) as usize;
			y = 0;
			if mp[y][x] {
				z2 = n;
				break;
			}
			n += 1;
		}
	}
	(z.unwrap().to_string(),z2.to_string())
}

use std::io::BufRead;
use std::cmp::Reverse;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut x = 0;
	let mut da = Vec::new();
	for i in f.lines() {
		if let Ok(j) = i.unwrap().parse::<i32>() {
			x += j;
		} else {
			da.push(x);
			x = 0
		}
	}
	da.push(x);
	da.sort_by_key(|&x| Reverse(x));
	let z = da[0];
	let z2:i32 = da[..3].iter().sum();
	(z.to_string(),z2.to_string())
}

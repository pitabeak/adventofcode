use std::io::BufRead;
use std::io;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let da = io::read_to_string(f).unwrap();
	let mut it = da.bytes().enumerate();
	let mut s = [0;4];
	let mut u = [0;14];
	let mut z = None;
	let mut z2 = 0;
	for (i,b) in da.bytes().enumerate() {
		if z.is_none() {
			s[i%4] = b;
			if i >= 4 && (1..4).all(|j| (0..j).all(|k| s[j]!=s[k])) {
				z = Some(i+1);
			}
		}
		u[i%14] = b;
		if i >= 14 && (1..14).all(|j| (0..j).all(|k| u[j]!=u[k])) {
			z2 = i+1;
			break;
		}
	}
	(z.unwrap().to_string(),z2.to_string())
}

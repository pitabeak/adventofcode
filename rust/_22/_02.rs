use std::io::BufRead;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut z = 0;
	let mut z2 = 0;
	for i in f.lines() {
		let i = i.unwrap();
		let i = i.as_bytes();
		let p = i[0] - b'A';
		let q = i[2] - b'X';
		z += (q + 1 + if q == (p+1)%3 {6} else if q == p {3} else {0}) as usize;
		z2 += (q*3 + (if q == 0 {p + 2} else if q == 1 {p} else {p + 1})%3 + 1) as usize;
	}
	(z.to_string(),z2.to_string())
}

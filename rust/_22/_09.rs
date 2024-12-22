use std::io::BufRead;
use std::collections::HashSet;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut pt:Vec<(i32,i32)> = Vec::new();
	for _ in 0..10 {
		pt.push((0,0));
	}
	let mut vz = HashSet::new();
	let mut v2 = HashSet::new();
	vz.insert((0,0));
	v2.insert((0,0));
	for i in f.lines() {
		let i = i.unwrap();
		let i = i.as_bytes();
		let a = i[0];
		let b = String::from_utf8_lossy(&i[2..]).parse::<usize>().unwrap();
		for _ in 0..b {
			let (mut x,mut y) = pt[0];
			(x,y) = match a {
				b'U' => (x,y-1),
				b'D' => (x,y+1),
				b'L' => (x-1,y),
				b'R' => (x+1,y),
				_ => panic!()
			};
			pt[0] = (x,y);
			for i in 1..10 {
				let (mut x1,mut y1) = pt[i];
				if (x-x1).abs()==2 || (y-y1).abs()==2  {
					x1 = x1 + (x-x1).signum();
					y1 = y1 + (y-y1).signum();
				}
				pt[i] = (x1,y1);
				(x,y) = (x1,y1);
			}
			vz.insert(pt[1]);
			v2.insert(pt[9]);
		}
	}
	let z = vz.len();
	let z2 = v2.len();
	(z.to_string(),z2.to_string())
}

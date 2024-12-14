use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut da:HashMap<u8,Vec<(i32,i32)>> = HashMap::new();
	let mut wd = 0i32;
	let mut ht = 0i32;
	for (y,i) in f.lines().enumerate() {
		let i = i.unwrap();
		wd = i.len() as i32;
		ht += 1;
		for (x,j) in i.bytes().enumerate() {
			if j != b'.' {
				da.entry(j).or_default().push((x as i32,y as i32));
			}
		}
	}
	let mut z = HashSet::new();
	let mut z2 = HashSet::new();
	for (_,v) in da {
		for i in 1..v.len() {
			for j in 0..i {
				let (mut a,mut b) = v[i];
				let (mut c,mut d) = v[j];
				let (dc,dd) = (c-a,d-b);
				z2.insert((a,b));
				let mut f = true;
				loop {
					a -= dc; b -= dd;
					if !(0<=a && a<wd && 0<=b && b<ht) { break; }
					if f {
						f = false;
						z.insert((a,b));
					}
					z2.insert((a,b));
				}
				z2.insert((c,d));
				let mut f = true;
				loop {
					c += dc; d += dd;
					if !(0<=c && c<wd && 0<=d && d<ht) { break; }
					if f {
						f = false;
						z.insert((c,d));
					}
					z2.insert((c,d));
				}
			}
		}
	}
	(z.len().to_string(),z2.len().to_string())
}

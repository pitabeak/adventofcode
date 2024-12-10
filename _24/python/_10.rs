// adventofcode.com 2024 day 10
use std::io;
use std::collections::HashSet;
use std::time::Instant;

pub fn main() {
	let da = io::read_to_string(io::stdin()).unwrap();
	let da:Vec<_> = da.lines().map(|s| s.as_bytes()).collect();
	let tm = Instant::now();
	let mx = da[0].len()-1;
	let my = da.len()-1;
	let mut z = 0;
	let mut z2 = 0;
	for y in 0..=my {
		for x in 0..=mx {
			if da[y][x] == b'0' {
				let mut vz = HashSet::new();
				for mut p in 0..262144 {
					let mut a = x;
					let mut b = y;
					let mut c = b'1';
					loop {
						match p&3 {
							0 => {
								if b==0 { break; }
								b -= 1;
							}
							1 => {
								if b==my { break; }
								b += 1;
							}
							2 => {
								if a==0 { break; }
								a -= 1;
							}
							3 => {
								if a==mx { break; }
								a += 1;
							}
							_ => todo!()
						}
						if da[b][a]!=c { break; }
						if c==b'9' {
							vz.insert((a,b));
							z2 += 1;
							break;
						}
						c += 1;
						p /= 4;
					}
				}
				z += vz.len();
			}
		}
	}
	println!("({} ms)",tm.elapsed().as_millis());
	println!("{z}");
	println!("{z2}");
}

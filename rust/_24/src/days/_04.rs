use std::io::BufRead;
use std::io;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let da = io::read_to_string(f).unwrap();
	let da:Vec<_> = da.lines().map(|s| s.as_bytes()).collect();
	let wd = da[0].len();
	let ht = da.len();
	let mut z = 0;
	let mut check = |s:&str| {
		if s=="XMAS" || s=="SAMX" { z += 1; }
	};
	for y in 0..ht {
		for x in 3..wd {
			let s = std::str::from_utf8(&da[y][x-3..x+1]).unwrap();
			check(s);
		}
	}
	for y in 3..ht {
		for x in 0..wd {
			let s:String = (0..4).map(|i| da[y-i][x] as char).collect();
			check(&s);
		}
		for x in 3..wd {
			let s:String = (0..4).map(|i| da[y-i][x-i] as char).collect();
			check(&s);
			let s:String = (0..4).map(|i| da[y-i][x-3+i] as char).collect();
			check(&s);
		}
	}
	let mut z2 = 0;
	for y in 2..ht {
		for x in 2..ht {
			if da[y-1][x-1] == b'A' {
				let a = da[y-2][x-2];
				let b = da[y-2][x];
				let c = da[y][x-2];
				let d = da[y][x];
				if (a==b'M' && d==b'S' || a==b'S' && d==b'M') && (b==b'M' && c==b'S' || b==b'S' && c==b'M') {
					z2 += 1;
				}
			}
		}
	}
	(z.to_string(),z2.to_string())
}

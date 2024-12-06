use std::io;
use std::collections::HashSet;
use std::io::Write;
use std::time::Instant;

fn check(da:&Vec<&[u8]>, wd:usize, ht:usize, mut c:i32, mut r:i32, p:i32, q:i32) -> Option<usize> {
	let mut dc = 0;
	let mut dr = -1;
	let mut vz = HashSet::new();
	vz.insert((c,r));
	let mut mv = HashSet::new();
	mv.insert((c,r,dc,dr));
	loop {
		c = c+dc; r += dr;
		if !(0<=c && c<wd as i32 && 0<=r && r<ht as i32) { break Some(vz.len()); }
		while da[r as usize][c as usize] == b'#' || c==p && r==q {
			c -= dc; r -= dr;
			(dc,dr) = (-dr,dc);
			c += dc; r += dr;
		}
		let m = (c,r,dc,dr);
		if mv.contains(&m) { break None; }
		mv.insert(m);
		vz.insert((c,r));
	}
}

pub fn main() {
	let tm = Instant::now();
	let da = io::read_to_string(io::stdin()).unwrap();
	let da:Vec<_> = da.lines().map(|s| s.trim().as_bytes()).collect();
	let wd = da[0].len();
	let ht = da.len();
	let (c,r) = (0..ht).find_map(|r| da[r].iter().position(|&i| i == b'^')
						.and_then(|c| Some((c as i32,r as i32)))).unwrap();
	let z = check(&da,wd,ht,c,r,-1,-1).unwrap();
	println!("{z:?}");
	print!("({}ms)",tm.elapsed().as_millis());
	let tm = Instant::now();
	let mut z = 0;
	for p in 0..wd {
		print!("."); io::stdout().flush().unwrap();
		for q in 0..ht {
			if da[q][p] == b'.' && check(&da,wd,ht,c,r,p as i32,q as i32).is_none() {
				z += 1;
			}
		}
	}
	println!("({}ms)",tm.elapsed().as_millis());
	println!("{z}");
}

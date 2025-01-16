use std::io::BufRead;
use std::collections::HashMap;

struct Board {
	bd:Vec<u8>,
	bt:usize,
	pc:Vec<Vec<u8>>,
	px:usize,
	wd:Vec<u8>,
	wx:usize,
}

impl Board {
	pub fn new(wd:String) -> Self {
		Board {
			bd:vec![255],
			bt:0,
			wd:wd.into_bytes(),
			wx:0,
			pc:vec![vec![15],vec![2,7,2],vec![7,4,4],vec![1,1,1,1],vec![3,3]],
			px:0,
		}
	}

	pub fn drop(&mut self) {
		let mut i = self.bt+4;
		while self.bd.len()<i+4 { self.bd.push(128); }
		let v = &self.pc[self.px%5];
		self.px += 1;
		let mut m = 2;
		loop {
			if self.wd[self.wx%self.wd.len()]==b'<' {
				if m>0 && (0..v.len()).all(|j| self.bd[i+j]&(v[j]<<(m-1))==0) {
					m -= 1;
				}
			} else if (0..v.len()).all(|j| self.bd[i+j]&(v[j]<<(m+1))==0) {
				m += 1;
			}
			self.wx += 1;
			if !(0..v.len()).all(|j| self.bd[i-1+j]&(v[j]<<m)==0) { break; }
			i -= 1;
		}
		(0..v.len()).for_each(|j| self.bd[i+j] |= v[j]<<m);
		i = self.bd.len()-1;
		while self.bd[i]==128 { i -= 1; }
		self.bt = i;
	}
}

enum Form {
	First(usize),
	Second(usize,usize,usize),
	Third()
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let p1 = 2022;
	let p2 = 1_000_000_000_000u64;
	let mut bd = Board::new(std::io::read_to_string(f).unwrap().trim_end().to_owned());
	let mut z = None;
	let mut z2 = None;
	let mut sn = HashMap::new();
	while !(z.is_some() && z2.is_some()) {
		if z2.is_none() {
			sn.entry((bd.px%5,bd.wx%bd.wd.len())).
				and_modify(|e| {
					let (h,n,d) = *e;
					if d==bd.bt-h && (p2-n as u64)%((bd.px-n) as u64)==0 {
						z2 = Some(h as u64+(p2-n as u64)/((bd.px-n) as u64)*d as u64);
					}
					*e = (bd.bt,bd.px,bd.bt-h);
				}).
				or_insert_with(|| (bd.bt,bd.px,0));
		}
		if bd.px==p1 { z = Some(bd.bt); }
		bd.drop();
	}
	(z.unwrap().to_string(),z2.unwrap().to_string())
}

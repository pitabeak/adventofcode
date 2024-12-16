//adventofcode.com 2024 Day 15
use std::io::BufRead;
use std::str::from_utf8;
use std::io::Lines;

#[derive(Debug)]
struct Warehouse {
	da:Vec<Vec<u8>>,
	wd:usize,
	ht:usize,
	gx:usize,
	gy:usize,
}

impl Warehouse {
	pub fn new<B:BufRead>(lines:&mut Lines<B>) -> Self {
		let mut da = Vec::new();
		while let Some(Ok(i)) = lines.next() {
			if i.len() == 0 { break; }
			da.push(i.as_bytes().to_vec());
		}
		let wd = da[0].len();
		let ht = da.len();
		let (gx,gy) = (0..wd*ht).map(|i| (i/ht,i%ht)).filter(|&(x,y)| da[y][x]==b'@').next().unwrap();
		Self{ da:da,wd:wd,ht:ht,gx:gx,gy:gy }
	}

	pub fn print(&self) {
		for i in &self.da {
			println!("{}",from_utf8(&i).unwrap());
		}
	}

	pub fn total_gps(&self) -> usize {
		(0..self.wd*self.ht).map(|i| (i/self.ht,i%self.ht)).
			filter_map(|(x,y)| (self.da[y][x]==b'O').then(|| 100*y+x)).sum()
	}

	pub fn robot_move(&mut self,dx:i32,dy:i32) {
		let mut p = self.gx;
		let mut q = self.gy;
		let mut f = true;
		loop {
			p = (p as i32 + dx) as usize;
			q = (q as i32 + dy) as usize;
			match self.da[q][p] {
				b'.' => { break; }
				b'#' => { f = false; break; }
				b'O' => {}
				_ => panic!()
			}
		}
		if f {
			self.da[q][p] = b'O';
			self.da[self.gy][self.gx] = b'.';
			self.gx = (self.gx as i32 + dx) as usize;
			self.gy = (self.gy as i32 + dy) as usize;
			self.da[self.gy][self.gx] = b'@';
		}
	}
}

#[derive(Debug)]
struct Wider {
	da:Vec<Vec<u8>>,
	wd:usize,
	ht:usize,
	gx:usize,
	gy:usize,
}

impl Wider {
	pub fn new(wh:&Warehouse) -> Self {
		let mut da = Vec::new();
		let mut s = String::new();
		for i in &wh.da {
			s.clear();
			s.extend(i.iter().map(|x| match x { b'.'=>"..", b'@'=>"@.", b'#'=>"##", b'O'=>"[]",
				_=>panic!() }));
			da.push(s.as_bytes().to_vec());
		}
		Self{ da:da,wd:wh.wd*2,ht:wh.ht,gx:wh.gx*2,gy:wh.gy }
	}

	pub fn print(self) {
		for i in self.da {
			println!("{}",from_utf8(&i).unwrap());
		}
	}

	pub fn total_gps(&self) -> usize {
		(0..self.wd*self.ht).map(|i| (i/self.ht,i%self.ht)).
			filter_map(|(x,y)| (self.da[y][x]==b'[').then(|| 100*y+x)).sum()
	}

	pub fn robot_move(&mut self,dx:i32,dy:i32) {
		if dy == 0 {
			let mut p = self.gx;
			let mut f = true;
			loop {
				p = (p as i32 + dx) as usize;
				match self.da[self.gy][p] {
					b'.' => { break; }
					b'#' => { f = false; break; }
					b'['|b']' => {}
					_ => panic!()
				}
			}
			if f {
				while p != self.gx {
					let p1 = (p as i32 - dx) as usize;
					self.da[self.gy][p] = self.da[self.gy][p1];
					p = p1;
				}
				self.da[self.gy][self.gx] = b'.';
				self.gx = (self.gx as i32 + dx) as usize;
			}
		} else {
			let q = (self.gy as i32 + dy) as usize;
			let mut f = true;
			match self.da[q][self.gx] {
				b'.' => {}
				b'#' => { f = false; }
				b'[' => {
					if self.can_push(self.gx,q,dy) { self.do_push(self.gx,q,dy); }
					else { f = false; }
				}
				b']' => {
					if self.can_push(self.gx-1,q,dy) { self.do_push(self.gx-1,q,dy); }
					else { f = false; }
				}
				_ => panic!()
			}
			if f {
				self.da[self.gy][self.gx] = b'.';
				self.gy = q;
				self.da[self.gy][self.gx] = b'@';
			}
		}
	}

	fn can_push(&self,x:usize,y:usize,dy:i32) -> bool {
		let q = (y as i32 + dy) as usize;
		(match self.da[q][x] {
			b'.' => true,
			b'#' => false,
			b'[' => { return self.can_push(x,q,dy); }
			b']' => self.can_push(x-1,q,dy),
			_ => panic!()
		})
		&&
		(match self.da[q][x+1] {
			b'.' => true,
			b'#' => false,
			b'[' => self.can_push(x+1,q,dy),
			_ => panic!()
		})
	}

	fn do_push(&mut self,x:usize,y:usize,dy:i32) {
		let q = (y as i32 + dy) as usize;
		match self.da[q][x] {
			b'.' => {}
			b'[' => self.do_push(x,q,dy),
			b']' => self.do_push(x-1,q,dy),
			_ => panic!()
		}
		match self.da[q][x+1] {
			b'.' => {}
			b'[' => self.do_push(x+1,q,dy),
			_ => panic!()
		}
		self.da[q][x] = b'[';
		self.da[q][x+1] = b']';
		self.da[y][x] = b'.';
		self.da[y][x+1] = b'.';
	}
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut it = f.lines();
	let mut wh = Warehouse::new(&mut it);
	let mut wd = Wider::new(&wh);
	while let Some(Ok(i)) = it.next() {
		for j in i.bytes() {
			let (dx,dy) = match j { b'^'=>(0,-1), b'v'=>(0,1), b'<'=>(-1,0), b'>'=>(1,0),
				_=>panic!() };
			wh.robot_move(dx,dy);
			wd.robot_move(dx,dy);
		}
	}
	let z = wh.total_gps();
	let z2 = wd.total_gps();
	(z.to_string(),z2.to_string())
}

/*
//run with cargo run <puzzleinput.txt
pub fn main() {
	let (p1,p2) = solve(io::stdin().lock());
	println!("{p1}\n{p2}");
}
*/

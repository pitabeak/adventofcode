use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug)]
struct Grid {
	dr:Vec<Vec<usize>>,
	dc:Vec<Vec<usize>>,
	gx:usize,
	gy:usize,
	mx:usize,
	my:usize,
}

impl Grid {
	pub fn new<T:BufRead>(br:T) -> Self {
		let mut z = Grid {
			dr:Vec::new(),
			dc:Vec::new(),
			gx:0,
			gy:0,
			mx:0,
			my:0,
		};
		for (y,i) in br.lines().enumerate() {
			let mut rw = Vec::new();
			for (x,j) in i.unwrap().bytes().enumerate() {
				if j==b'#' {
					rw.push(x);
					while z.dc.len() <= x { z.dc.push(Vec::new()); }
					z.dc[x].push(y);
				} else if j==b'^' {
					z.gx = x; z.gy = y;
				}
			}
			z.dr.push(rw);
		}
		z.mx = z.dc.len()-1;
		z.my = z.dr.len()-1;
		z
	}

	pub fn count_pos(&self) -> usize {
		let mut x = self.gx;
		let mut y = self.gy;
		let mut vz = HashSet::new();
		vz.insert((x,y));
		loop {

			let i = self.dc[x].iter().take_while(|&&j| j < y).count();
			let y1 = if i > 0 { self.dc[x][i-1]+1 } else { 0 };
			while y > y1 { y -= 1; vz.insert((x,y)); }
			if y == 0 { break; }

			let i = self.dr[y].iter().take_while(|&&j| j < x).count();
			let x1 = if i < self.dr[y].len() { self.dr[y][i]-1 } else { self.mx };
			while x < x1 { x += 1; vz.insert((x,y)); }
			if x == self.mx { break; }

			let i = self.dc[x].iter().take_while(|&&j| j < y).count();
			let y1 = if i < self.dc[x].len() { self.dc[x][i]-1 } else { self.my };
			while y < y1 { y += 1; vz.insert((x,y)); }
			if y == self.my { break; }

			let i = self.dr[y].iter().take_while(|&&j| j < x).count();
			let x1 = if i > 0 { self.dr[y][i-1]+1 } else { 0 };
			while x > x1 { x -= 1; vz.insert((x,y)); }
			if x == 0 { break; }
		}
		vz.len()
	}

	pub fn is_loop(&self) -> bool {
		let mut x = self.gx;
		let mut y = self.gy;
		let mut vz = HashSet::new();
		loop {

			let i = self.dc[x].iter().take_while(|&&j| j < y).count();
			if i == 0 { break; }
			let y1 = self.dc[x][i-1]+1;
			if y != y1 {
				y = y1;
				if vz.contains(&(x,y)) { return true; }
				vz.insert((x,y));
			}

			let i = self.dr[y].iter().take_while(|&&j| j < x).count();
			if i == self.dr[y].len() { break; }
			let x1 = self.dr[y][i]-1;
			if x != x1 {
				x = x1;
				if vz.contains(&(x,y)) { return true; }
				vz.insert((x,y));
			}

			let i = self.dc[x].iter().take_while(|&&j| j < y).count();
			if i == self.dc[x].len() { break; }
			let y1 = self.dc[x][i]-1;
			if y != y1 {
				y = y1;
				if vz.contains(&(x,y)) { return true; }
				vz.insert((x,y));
			}

			let i = self.dr[y].iter().take_while(|&&j| j < x).count();
			if i == 0 { break; }
			let x1 = self.dr[y][i-1]+1;
			if x != x1 {
				x = x1;
				if vz.contains(&(x,y)) { return true; }
				vz.insert((x,y));
			}
		
		}
		false
	}

	pub fn count_loop(&mut self) -> usize {
		let mut z = 0;
		for y in 0..=self.my {
			for x in 0..=self.mx {
				if !self.dr[y].iter().any(|&i| i==x) {
					let i = self.dr[y].iter().take_while(|&&k| k < x).count();
					let j = self.dc[x].iter().take_while(|&&k| k < y).count();
					self.dr[y].insert(i,x);
					self.dc[x].insert(j,y);
					if self.is_loop() { z += 1; }
					self.dr[y].remove(i);
					self.dc[x].remove(j);
				}
			}
		}
		z
	}
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut grid = Grid::new(f);
	(grid.count_pos().to_string(),grid.count_loop().to_string())
}

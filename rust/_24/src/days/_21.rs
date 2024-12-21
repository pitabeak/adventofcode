use std::io::BufRead;
use core::fmt::Debug;

trait Keypad {
	fn command(&mut self,c:u8) -> Option<u8>;
	fn dbg(&self) -> String;
}

impl Debug for dyn Keypad {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.dbg())
    }
}

struct Numeric {
	x:usize,
	y:usize,
}

impl Numeric {
	const GRID:[[u8;3];4] = [[b'7',b'8',b'9'],[b'4',b'5',b'6'],[b'1',b'2',b'3'],[b' ',b'0',b'A']];

	pub fn new() -> Self {
		Self{ x:2,y:3 }
	}
}

impl Keypad for Numeric {
	fn command(&mut self,c:u8) -> Option<u8> {
		match c {
			b'A' => Some(Numeric::GRID[self.y][self.x]),
			b'^' => if self.y > 0 { self.y -= 1; None } else { panic!("NUMERIC FAULT"); }
			b'v' => if self.y < 3 { self.y += 1; None } else { panic!("NUMERIC FAULT"); }
			b'>' => if self.x < 2 { self.x += 1; None } else { panic!("NUMERIC FAULT"); }
			b'<' => if if self.y < 3 {self.x > 0} else {self.x > 1} { self.x -= 1; None }
				else { panic!("NUMERIC FAULT"); }
			_ => panic!()
		}
	}

	fn dbg(&self) -> String { format!("({})",Numeric::GRID[self.y][self.x] as char) }
}

struct Directional {
	x:usize,
	y:usize,
}

impl Directional {
	const GRID:[[u8;3];2] = [[b' ',b'^',b'A'],[b'<',b'v',b'>']];

	pub fn new() -> Self {
		Self{ x:2,y:0 }
	}
}

impl Keypad for Directional {
	fn command(&mut self,c:u8) -> Option<u8> {
		match c {
			b'A' => Some(Directional::GRID[self.y][self.x]),
			b'^' => if self.y > 0 { self.y -= 1; None } else { panic!("NUMERIC FAULT"); }
			b'v' => if self.y < 1 { self.y += 1; None } else { panic!("NUMERIC FAULT"); }
			b'>' => if self.x < 2 { self.x += 1; None } else { panic!("NUMERIC FAULT"); }
			b'<' => if if self.y > 0 {self.x > 0} else {self.x > 1} { self.x -= 1; None }
				else { panic!("NUMERIC FAULT"); }
			_ => panic!()
		}
	}

	fn dbg(&self) -> String { format!("({})",Directional::GRID[self.y][self.x] as char) }
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let da:Vec<_> = f.lines().map(|s| s.unwrap().as_bytes().to_vec()).collect();
	let mut ky:[Box<dyn Keypad>;3] = [
		Box::new(Directional::new()),
		Box::new(Directional::new()),
		Box::new(Numeric::new()),
	];
	let ip = b"<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A";
	//let ip = b"<A^A>^^AvvvA";
	//let ip = b"v<<A>>^A<A>AvA<^AA>A<vAAA>^A";
	for &c in ip.iter() {
		println!("{} {:?} {:?} {:?}",c as char,ky[0],ky[1],ky[2]);
		let mut c = c;
		let mut f = true;
		for t in ky.iter_mut() {
			if let Some(c1) = t.command(c) { c = c1; }
			else { f = false; break; }
		}
		if f { println!(">>{}",c as char); }
	}
	(0.to_string(),0.to_string())
}

use std::io::BufRead;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Debug)]
pub struct Towels<'a> {
	sr:&'a Vec<&'a str>,
	mm:HashMap<usize,usize>,
	ta:&'a str,
}

impl<'a> Towels<'a> {
	fn find(sr:&Vec<&'a str>,ta:&str) -> usize {
		let mut tw = Towels{ sr:sr,ta:ta,mm:HashMap::new() };
		tw.find1(0)
	}

	pub fn find1(&mut self,i:usize) -> usize {
		if i == self.ta.len() { return 1; }
		if let Entry::Occupied(e) = self.mm.entry(i) { return *e.get(); }
		let x = self.sr.iter().map(|u| if self.ta[i..].starts_with(u) {self.find1(i+u.len())} else {0}).sum();
		self.mm.insert(i,x);
		x
	}
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut it = f.lines();
	let s = it.next().unwrap().unwrap();
	let sr:Vec<_> = s.split(", ").collect();
	let mut z = 0;
	let mut z2 = 0;
	for s in it.skip(1).map(|s| s.unwrap()) {
		let i = Towels::find(&sr,&s);
		if i > 0 { z += 1; }
		z2 += i;
	}
	(z.to_string(),z2.to_string())
}

use std::io::BufRead;
use std::io;
use std::collections::HashMap;

struct Memoized {
	mm:HashMap<(usize,usize),usize>
}

impl Memoized {
	pub fn new() -> Memoized {
		let mut z = Memoized{ mm:HashMap::new() };
		z.mm.insert((0,1),1);	// specific trivial cases
		z
	}

	pub fn eval(&mut self, (k,n):(usize,usize)) -> usize {
		if n == 0 {				// class of trivial cases
			1
		} else if let Some(&v) = self.mm.get(&(k,n)) {
			v
		} else {
			let v = if k > 0 {	// compute value
				let j = k.ilog10();
				if j%2 == 0 {
					self.eval((k*2024,n-1))
				} else {
					let j = 10usize.pow(j/2+1);
					self.eval((k/j,n-1)) + self.eval((k%j,n-1))
				}
			} else {
				self.eval((1,n-1))
			};
			self.mm.insert((k,n),v);
			v
		}
	}
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let da:Vec<usize> = io::read_to_string(f).unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
	let mut mm = Memoized::new();
	let mut z = 0;
	let mut z2 = 0;
	for i in da {
		z += mm.eval((i,25));
		z2 += mm.eval((i,75));
	}
	(z.to_string(),z2.to_string())
}

use std::io::BufRead;
use std::io;
use std::collections::HashMap;

struct Stones {
	mm:HashMap<(usize,usize),usize>
}

impl Stones {
	pub fn new() -> Stones {
		Stones{ mm:HashMap::new() }
	}

	pub fn count(&mut self, (n,k):(usize,usize)) -> usize {
		if n == 0 {				//the terminating case, put here to reduce memory usage
			return 1;
		}
		if let Some(&v) = self.mm.get(&(n,k)) {
			return v;
		}
		let v =
			if k == 0 {			//find the result and store it
				self.count((n-1,1))
			} else {
				let j = k.ilog10();	//number of digits - 1
				if j%2 == 0 {
					self.count((n-1,k*2024))
				} else {
					let j = 10usize.pow((j+1)/2);
					self.count((n-1,k/j)) + self.count((n-1,k%j))
				}
			};
		self.mm.insert((n,k),v);
		v
	}
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let da:Vec<usize> = io::read_to_string(f).unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
	let mut st = Stones::new();
	let z:usize = da.iter().map(|&i| st.count((25,i))).sum();
	let z2:usize = da.iter().map(|&i| st.count((75,i))).sum();
	(z.to_string(),z2.to_string())
}

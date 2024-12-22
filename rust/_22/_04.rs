use std::io::BufRead;
use regex::Regex;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let re = Regex::new(r"\d+").unwrap();
	let mut a = Vec::with_capacity(4);
	let mut z = 0;
	let mut z2 = 0;
	for i in f.lines().map(|s| s.unwrap()) {
		a.clear();
		a.extend(re.find_iter(&i).map(|m| m.as_str().parse::<i32>().unwrap()));
		if a[0]<=a[2] && a[3]<=a[1] || a[2]<=a[0] && a[1]<=a[3] { z += 1; }
		if a[0]<=a[2] && a[2]<=a[1] || a[2]<=a[0] && a[0]<=a[3] ||
			a[0]<=a[3] && a[3]<=a[1] || a[2]<=a[1] && a[1]<=a[3]
			{ z2 += 1; }
	}
	(z.to_string(),z2.to_string())
}

use std::io::BufRead;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut a = Vec::new();
	let mut b = Vec::new();
	let fs = 100000;
	let sd = 40000000;
	for i in f.lines().map(|s| s.unwrap()) {
		if i == "$ cd .." {
			let x = a.remove(0);
			a[0] += x;
			b.push(x);
		} else if i.starts_with("$ cd") {
			a.insert(0,0);
		} else if let Ok(x) = i.split_once(' ').unwrap().0.parse::<usize>() {
			a[0] += x;
		}
	}
	while a.len() > 1 {
		let x = a.remove(0);
		a[0] += x;
		b.push(x);
	}
	b.push(a[0]);
	let sz = a[0] - sd;
	let z = b.iter().filter(|&&x| x <= fs).sum::<usize>();
	let z2 = b.iter().filter(|&&x| x >= sz).min().unwrap();
	(z.to_string(),z2.to_string())
}

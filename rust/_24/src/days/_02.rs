use std::io::BufRead;

fn issafe(a: &Vec<i32>) -> bool {
	let (n,x) = if a[1]>a[0] { (1,3) } else { (-3,-1) };
	let mut b = a.iter();
	let mut j = *b.next().unwrap();
	for &i in b {
		if !(j+n<=i && i<=j+x) { return false; }
		j = i
	}
	true
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut z = 0;
	let mut z2 = 0;
	for i in f.lines() {
		let a:Vec<i32> = i.unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
		if issafe(&a) {
			z += 1;
			z2 += 1;
		} else {
			for i in 0..a.len() {
				let mut b = a.clone();
				b.remove(i);
				if issafe(&b) {
					z2 += 1;
					break;
				}
			}
		}
	}
	(z.to_string(),z2.to_string())
}

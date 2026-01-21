use std::fs::read_to_string;

fn main() {
	let fi = read_to_string("data.txt").unwrap();
	let mut n1 = 0;
	let mut n2 = 0;
	for (a,b,c) in fi.lines().map(|s| {
			let v:Vec<i32> = s.split('x').map(|x| x.parse().unwrap()).collect();
			(v[0],v[1],v[2])
		}) {
		let (x,y,z) = (a*b,a*c,b*c);
		let m = x.min(y).min(z);
		n1 += 2*(x+y+z)+m;
		let (x,y,z) = (a+b,a+c,b+c);
		let m = x.min(y).min(z);
		n2 += 2*m + a*b*c;
	}
	println!("{n1} {n2}");
}

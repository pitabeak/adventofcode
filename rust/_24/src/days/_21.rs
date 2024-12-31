use std::io::BufRead;
use cached::proc_macro::cached;

fn pos(b:u8) -> (usize,usize) {
	let i = b"789456123_0A<v>".iter().position(|&x| x==b).unwrap();
	(i/3,i%3)
}

fn path(a:u8,b:u8) -> Vec<u8> {
	let (y1,x1) = pos(a);
	let (y2,x2) = pos(b);
	const z:&str = "";
	let s = format!("{z:><0$}{z:v<1$}{z:0<2$}{z:<<3$}",
		x2.checked_sub(x1).unwrap_or_default(),
		y2.checked_sub(y1).unwrap_or_default(),
		y1.checked_sub(y2).unwrap_or_default(),
		x1.checked_sub(x2).unwrap_or_default());
	let mut s = s.as_bytes().to_vec();
	if !(y1==3 && x2==0 || y2==3 && x1==0) {
		let l = s.len();
		for i in 0..l/2 {
			s.swap(i,l-1-i);
		}
	}
	s
}

#[cached]
fn size(s:Vec<u8>,d:i32) -> usize {
	let l = s.len();
	if d<0 { return l+1; }
	let mut r = 0;
	if l > 0 {
		r += size(path(b'A',s[0]),d-1) + size(path(s[l-1],b'A'),d-1);
		for i in 1..l {
			r += size(path(s[i-1],s[i]),d-1);
		}
	} else {
		r += size(path(b'A',b'A'),d-1);
	}
	r
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut z = 0;
	let mut z2 = 0;
	for s in f.lines() {
		let s = s.unwrap();
		let s = s.strip_suffix("A").unwrap();
		let n = s.parse::<usize>().unwrap();
		z += n * size(s.as_bytes().to_vec(),2);
		z2 += n * size(s.as_bytes().to_vec(),25);
	}
	println!("(I couldn't wrap my head around this problem, so this solution is copied from 4HbQ's\npython answer on the reddit aoc solutions thread, translated to rust.)");

	(z.to_string(),z2.to_string())
}

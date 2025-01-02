use std::io::BufRead;
use cached::proc_macro::cached;

fn pos(b:u8) -> (usize,usize) {
	let i = b"789456123_0A<v>".iter().position(|&x| x==b).unwrap();
	(i%3,i/3)
}

#[cached]
fn find(a:u8,b:u8,m:u8) -> usize {
	if a==b { return 1; }
	let (x1,y1) = pos(a);
	let (x2,y2) = pos(b);
	if m==0 { return x1.abs_diff(x2) + y1.abs_diff(y2) + 1; }
	//the order here is important, but I don't know why
	let (c,i,d,j) = if x1==0 && y2==3 {
		if y2>y1 { (b'>',x2,b'v',y2-y1) }
		else { (b'>',x2,b'0',y1-y2) }
	} else if x2==0 && y1==3 {
		if y2>y1 { (b'v',y2-y1,b'<',x1) }
		else { (b'0',y1-y2,b'<',x1) }
	} else if x2>x1 {
		if y2>y1 { (b'v',y2-y1,b'>',x2-x1) }
		else { (b'0',y1-y2,b'>',x2-x1) }
	} else {
		if y2>y1 { (b'<',x1-x2,b'v',y2-y1) }
		else { (b'<',x1-x2,b'0',y1-y2) }
	};
	let mut f = b'A';
	let mut z = 0;
	for _ in 0..i {
		z += find(f,c,m-1);
		f = c;
	}
	for _ in 0..j {
		z += find(f,d,m-1);
		f = d;
	}
	z += find(f,b'A',m-1);
	z
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut z = 0;
	let mut z2 = 0;
	for s in f.lines() {
		let s = s.unwrap();
		let n = s.strip_suffix("A").unwrap().parse::<usize>().unwrap();
		let mut r = 0;
		let mut f = b'A';
		for &b in s.as_bytes() {
			r += find(f,b,2);
			f = b;
		}
		z += r * n;
		r = 0;
		f = b'A';
		for &b in s.as_bytes() {
			r += find(f,b,25);
			f = b;
		}
		z2 += r * n;
	}
	(z.to_string(),z2.to_string())
}

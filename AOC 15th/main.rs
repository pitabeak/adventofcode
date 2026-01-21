use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let fi = read_to_string("data.txt").unwrap();
	let mut x = 0;
	let mut y = 0;
	let (mut x1,mut y1) = (0,0);
	let (mut x2,mut y2) = (0,0);
	let mut m = HashSet::new();
	let mut m1 = HashSet::new();
	m.insert((x,y));
	m1.insert((x,y));
	let mut f = true;
	for c in fi.chars() {
		fly(&mut x,&mut y,c);
		m.insert((x,y));
		if f {
			fly(&mut x1,&mut y1,c);
			m1.insert((x1,y1));
		} else {
			fly(&mut x2,&mut y2,c);
			m1.insert((x2,y2));
		}
		f = !f;
	}
	let (n1,n2) = (m.len(),m1.len());
	println!("{n1} {n2}");
}

fn fly(x:&mut i32, y:&mut i32, c:char) {
	match c {
		'^'=>*y-=1,
		'v'=>*y+=1,
		'<'=>*x-=1,
		'>'=>*x+=1,
		_=>panic!()
	}
}


use std::io;

pub fn main() {
	let s = io::read_to_string(io::stdin()).unwrap();
	let s = s.trim().as_bytes();
	let mut lf = 0;
	let mut rt = s.len()+(s.len()&1);
	let mut rn = 0;
	let mut i = 0;
	let mut z = 0;
	'l1:
	while lf < rt {
		for _ in b'0'..s[lf] {
			z += lf/2*i;
			i += 1;
		}
		lf += 2;
		for _ in b'0'..s[lf-1] {
			while rn == 0 {
				if lf >= rt { break 'l1; }
				rt -= 2;
				rn = s[rt]-b'0';
			}
			rn -= 1;
			z += rt/2*i;
			i += 1;
		}
	}
	while rn > 0 {
		rn -= 1;
		z += rt/2*i;
		i += 1;
	}
	println!("{z}");
	let mut s = Vec::from(s);
	let rx = s.len()+(s.len()&1)-2;
	i = 0;
	z = 0;
	for lf in (0..s.len()).step_by(2) {
		if s[lf] < b'A' {
			for _ in b'0'..s[lf] {
				z += lf/2*i;
				i += 1;
			}
		} else {
			i += (s[lf]-b'A') as usize;
		}
		if lf+1 >= s.len() { break; }
		let mut sp = s[lf+1];
		while sp != b'0' {
			let mut x = 0;
			rt = rx;
			while lf < rt {
				if s[rt] <= sp { x = s[rt]; break; }
				rt -= 2;
			}
			if x == 0 { i += (sp-b'0') as usize; break; }
			s[rt] = b'A'+(x-b'0');
			for _ in b'0'..x {
				z += rt/2*i;
				i += 1;
			}
			sp -= x-b'0';
		}
	}
	println!("{z}");
}

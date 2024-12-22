use std::io::BufRead;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut z = 0;
	let mut z2 = 0;
	let mut a = Vec::new();
	let mut b = Vec::new();
	for (k,i) in f.lines().enumerate() {
		let i = i.unwrap();
		let i = i.as_bytes();
		let h = i.len()/2;
		for j in h..i.len() {
			let c = i[j];
			if i.iter().position(|&x| x==c).unwrap() < h {
				z += if c < b'a' { c - b'A' + 27 } else { c - b'a' + 1 } as usize;
				break;
			}
		}
		match k%3 {
			0 => a = i.to_vec(),
			1 => b = i.to_vec(),
			2 => {
				for &j in i {
					if a.iter().any(|&x| x==j) && b.iter().any(|&x| x==j) {
						z2 += if j < b'a' { j - b'A' + 27 } else { j - b'a' + 1 } as usize;
						break;
					}
				}
			}
			_ => todo!()
		}
	}
	(z.to_string(),z2.to_string())
}

// adventofcode.com 2024 day 17
use std::io::BufRead;
use std::io;

fn g(rg:&Vec<i64>,cd:&Vec<u8>) -> i64 {
	let mut rg = rg.to_vec();
	let mut pc = 0;
	let mut z = 0;
	while pc < cd.len() {
		let i = cd[pc]; pc += 1;
		let o = cd[pc]; pc += 1;
		match i {
			0 => rg[0] = rg[0] >> (if o >= 4 { rg[(o-4) as usize] } else { o as i64 }),
			1 => rg[1] ^= o as i64,
			2 => rg[1] = (if o >= 4 { rg[(o-4) as usize] } else { o as i64 }) & 7,
			3 => if rg[0] != 0 { pc = o as usize; }
			4 => rg[1] ^= rg[2],
			5 => z = z*8 + ((if o >= 4 { rg[(o-4) as usize] } else { o as i64 }) & 7),
			6 => rg[1] = rg[0] >> (if o >= 4 { rg[(o-4) as usize] } else { o as i64 }),
			7 => rg[2] = rg[0] >> (if o >= 4 { rg[(o-4) as usize] } else { o as i64 }),
			_ => panic!()
		}
	}
	z
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let da = io::read_to_string(f).unwrap();
	let re = regex::Regex::new(r"\d+").unwrap();
	let da:Vec<i32> = re.find_iter(&da).map(|m| m.as_str().parse().unwrap()).collect();
	let mut rg:Vec<_> = da.iter().take(3).map(|&x| x as i64).collect();
	let cd:Vec<_> = da.iter().skip(3).map(|&x| x as u8).collect();
	let rz = cd.iter().fold(0,|ac:i64,&x| ac*8+x as i64);

	//assume the first number in the output is not zero
	let z = g(&rg,&cd);
	let z:Vec<_> = format!("{z:o}").chars().map(|c| c.to_string()).collect();
	let z = z.join(",");

	//The puzzle code is a function from octal integers to octal integers.
	//If the first N digits of two inputs are the same, then the last N digits
	//of the outputs are the same (I don't know why).
	let mut z2 = i64::MAX;
	let mut x = 0;
	let mut ic = 1 << 3*(cd.len()-1);
	let mut mk = 7;
	loop {
		x += ic;
		if x&(7*ic) == 0 {
			if mk == 7 { break; }
			mk >>= 3;
			ic <<= 3;
		}
		rg[0] = x;
		if g(&rg,&cd)&mk == rz&mk {
			if ic > 1 {
				mk = (mk<<3) | 7;
				ic >>= 3;
			} else {
				z2 = z2.min(x);
			}
		}
	}
	(z,z2.to_string())
}

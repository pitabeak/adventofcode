use std::fs::read_to_string;

fn main() {
	let fi  = read_to_string("data.txt").unwrap();
	let mut i = 0;
	let mut j = 1;
	let mut k = None;
	for c in fi.chars() {
		if c=='(' { i += 1; }
		else { i -= 1; }
		if k.is_none() && i<0 { k = Some(j); }
		j += 1;
	}
	println!("{i} {}",k.unwrap());
}

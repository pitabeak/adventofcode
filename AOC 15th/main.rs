const PZ:&str = "ckczppom";

fn main() {
	let (mut n1,mut n2) = (None,0);
	for i in 0.. {
		let s = md5(&format!("{PZ}{i}"));
		if s.starts_with("00000") {
			if n1.is_none() { n1 = Some(i); }
			if s.starts_with("000000") { n2 = i;  break; }
		}
	}
	println!("{} {}",n1.unwrap(),n2);
}

fn md5(s:&String) -> String {
	format!("{:?}",md5::compute(s.as_bytes()))
}

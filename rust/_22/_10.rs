use std::io::BufRead;

fn cycle(n:i32,x:i32,z:&mut i32,d:&mut Vec<u8>) {
	if (n - 20)%40 == 0 {
		*z += n*x;
	}
	let c = (n-1)%40;
	d[(n-1) as usize] = if x-1<=c && c<=x+1 { b'#' } else { b'.' };
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut n = 1;
	let mut x = 1;
	let mut z = 0;
	let mut d = vec![b' ';240];
	for i in f.lines() {
		let i = i.unwrap();
		if let Some(s) = i.strip_prefix("addx ") {
			cycle(n,x,&mut z,&mut d); n += 1;
			cycle(n,x,&mut z,&mut d); n += 1;
			x += s.parse::<i32>().unwrap();
		} else {
			cycle(n,x,&mut z,&mut d); n += 1;
		}
	}
	for i in (0..240).step_by(40) {
		println!("{}",String::from_utf8_lossy(&d[i..i+40]));
	}
	(z.to_string(),"RGZEHURK".to_string())
}

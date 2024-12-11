use std::env;
use std::fs::File;
use std::io;
use std::io::IsTerminal;
use std::io::BufReader;
use std::io::BufRead;
use std::time::Instant;

mod _01;
mod _02;
mod _03;
mod _05;
mod _06;
mod _09;
mod _10;

fn main() {
	let b = if env::args().len()>1 {
		env::args().skip(1).map(|s| s.parse().unwrap()).collect()
	} else {
		vec![1,2,3,5,6,9,10]
	};
	let mut tr = io::stdin().is_terminal();
	for i in b {
		let f:Box<dyn BufRead> = if tr {
			Box::new(BufReader::new(File::open(format!("dat/_{i:02}.txt")).unwrap()))
		} else {
			Box::new(BufReader::new(io::stdin().lock()))
		};
		let tm = Instant::now();
		let (p1,p2) =
		match i {
			1 => { _01::solve(f) }
			2 => { _02::solve(f) }
			3 => { _03::solve(f) }
			5 => { _05::solve(f) }
			6 => { _06::solve(f) }
			9 => { _09::solve(f) }
			10 => { _10::solve(f) }
			_ => todo!()
		};
		println!("\nDay {i} ({}ms)\n{p1}\n{p2}",tm.elapsed().as_millis());
		tr = true;
	}
}

#![allow(warnings)]
use std::env;
pub mod days;

use std::fs::File;
use std::io;
use std::io::IsTerminal;
use std::io::BufReader;
use std::io::BufRead;
use std::time::Instant;

fn main() {
	let a = [
		days::_01::solve,
		days::_02::solve,
		days::_03::solve,
		days::_04::solve,
		days::_05::solve,
		days::_06::solve,
		days::_07::solve,
		days::_08::solve,
		days::_09::solve,
		days::_10::solve,
		days::_11::solve,
		days::_12::solve,
		days::_13::solve,
		days::_14::solve,
		days::_15::solve,
		days::_16::solve,
		days::_17::solve,
		days::_18::solve,
		days::_19::solve,
		days::_20::solve,
		days::_21::solve,
		days::_22::solve,
		days::_23::solve,
		days::_24::solve,
		days::_25::solve,
	];
	let b = if env::args().len()>1 {
		env::args().skip(1).map(|s| s.parse().unwrap()).collect()
	} else {
		(1..=a.len()).collect::<Vec<usize>>()
	};
	let mut tr = io::stdin().is_terminal();
	for i in b {
		let f:Box<dyn BufRead> = if tr {
			Box::new(BufReader::new(File::open(format!("dat/_{i:02}.txt")).unwrap()))
		} else {
			Box::new(BufReader::new(io::stdin().lock()))
		};
		let tm = Instant::now();
		println!("\n --- Day {i} ---");
		let (p1,p2) = a[i-1](f);
		println!("({}ms)\n\n{p1}\n{p2}",tm.elapsed().as_millis());
		tr = true;
	}
}

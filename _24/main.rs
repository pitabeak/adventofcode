mod _01;
mod _02;
mod _03;
mod _05;
mod _06;
use std::env;

fn main() {
	match env::args().nth(1).unwrap().as_str() {
		"_01" => { _01::main(); }
		"_02" => { _02::main(); }
		"_03" => { _03::main(); }
		"_05" => { _05::main(); }
		"_06" => { _06::main(); }
		_ => todo!()
	}
}

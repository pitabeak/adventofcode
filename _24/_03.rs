use std::io;
use regex::Regex;
use std::io::BufRead;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let da = io::read_to_string(f).unwrap();
	let re = Regex::new(r"mul\((\d+),(\d+)\)|(do)((?:n\'t)?)\(\)").unwrap();
	let mut f = true;
	let mut z = 0;
	let mut z2 = 0;
	for [i,j] in re.captures_iter(&da).map(|c| c.extract::<2>().1) {
		if i == "do" {
			f = j == ""
		} else {
			let x = i.parse::<i32>().unwrap() * j.parse::<i32>().unwrap();
			z += x;
			if f { z2 += x; }
		}
	}
	(z.to_string(),z2.to_string())
}

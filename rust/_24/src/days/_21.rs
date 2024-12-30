use std::io::BufRead;
use std::collections::{HashMap,BinaryHeap};
use std::collections::hash_map::Entry;
use std::cmp::Reverse;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let dl:HashMap<u8,(usize,usize)> = [(0,(2,1)),(1,(1,1)),(2,(0,1)),(3,(1,0)),(4,(2,0))].
		into_iter().collect();
	let mut ds:Vec<HashMap<(u8,u8),usize>> = (0..3).map(|_| HashMap::new()).collect();
	for i in 1..5 {
		for j in 0..i {
			let (x1,y1) = dl[&i];
			let (x2,y2) = dl[&j];
			ds[0].insert((i,j),x1.abs_diff(x2)+y1.abs_diff(y2));
		}
	}
	(0.to_string(),0.to_string())
}

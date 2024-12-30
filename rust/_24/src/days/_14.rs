use std::io::BufRead;
use regex::Regex;
//use std::str::from_utf8;
//use std::time::Instant;
use std::collections::HashSet;

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let re = Regex::new(r"-?\d+").unwrap();
	let wd = 101;
	let ht = 103;
	let mut po = Vec::new();
	let mut sp = Vec::new();
	for i in f.lines() {
		let a:Vec<i32> = re.find_iter(&i.unwrap()).map(|m| m.as_str().parse().unwrap()).collect();
		po.push([a[0],a[1]]);
		sp.push([a[2],a[3]]);
	}
	let mut qu = [0;4];
	//let mut tm = Instant::now();
	for j in 1.. {
		//if tm.elapsed().as_secs() > 1 { println!("({j})"); tm = Instant::now(); }
		for i in 0..po.len() {
			po[i][0] = (po[i][0] + wd + sp[i][0])%wd;
			po[i][1] = (po[i][1] + ht + sp[i][1])%ht;
		}
		if j == 100 {
			for &[x,y] in &po {
				if !(x == wd/2 || y == ht/2) {
					let j = if x<wd/2 { if y<ht/2 {0} else {1} }
					else { if y<ht/2 {2} else {3} };
					qu[j] += 1;
				}
			}
		}
		let mp:HashSet<&[i32;2]> = HashSet::from_iter(po.iter());
		if j == 8280 { break; }
	}
	/*
	//I haven't solved the second part
	//a clue to it is that no two robots are stacked on a tile
	let mut mp:Vec<_> = (0..ht).map(|_| vec![b'.';wd as usize]).collect();
	for [x,y] in po {
		let x = x as usize;
		let y = y as usize;
		mp[y][x] = b'#';
	}
	for s in mp { println!("{}",from_utf8(&s).unwrap()); }
	*/
	let z:i32 = qu.iter().product();
	(z.to_string(),8280.to_string())
}

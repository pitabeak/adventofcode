use std::io::BufRead;
use std::io;
use std::collections::{HashMap,HashSet,BinaryHeap};
use std::cmp::Reverse;

fn walk(x:i16,y:i16,d:u8) -> (i16,i16) {
	match d { 0=>(x+1,y),1=>(x,y+1),2=>(x-1,y),3=>(x,y-1),_=>panic!() }
}

fn walkback(x:i16,y:i16,d:u8) -> (i16,i16) {
	match d { 0=>(x-1,y),1=>(x,y-1),2=>(x+1,y),3=>(x,y+1),_=>panic!() }
}

fn find(n:usize,p:(i16,i16,u8),ds:&HashMap<(i16,i16,u8),usize>,vz:&mut HashSet<(i16,i16,u8)>) {
	if !vz.contains(&p) {
		vz.insert(p);
		if n == 0 { return; }
		if n >= 1000 {
			let q = (p.0,p.1,p.2^1);
			let m = n-1000;
			if *ds.get(&q).unwrap() == m { find(m,q,ds,vz); }
			let q = (p.0,p.1,p.2^3);
			if *ds.get(&q).unwrap() == m { find(m,q,ds,vz); }
			if n >= 2000 {
				let q = (p.0,p.1,p.2^2);
				let m = n-2000;
				if *ds.get(&q).unwrap() == m { find(m,q,ds,vz); }
			}
		}
		let (x,y) = walkback(p.0,p.1,p.2);
		let q = (x,y,p.2);
		let m = n-1;
		if ds.get(&q).is_some_and(|&u| u == m) { find(m,q,ds,vz); }
	}
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let da = io::read_to_string(f).unwrap();
	let da:Vec<_> = da.lines().map(|s| s.as_bytes()).collect();
	let wd = da[0].len() as i16;
	let ht = da.len() as i16;
	let mut ds = HashMap::new();
	let mut qu = BinaryHeap::new();

	let (n,p) = (0usize,(1,ht-2,0u8));
	ds.insert(p,n);
	qu.push(Reverse((n,p)));
	while let Some(Reverse((n,(x,y,d)))) = qu.pop() {
		let (u,v) = walk(x,y,d);
		if da[u as usize][v as usize] != b'#' {
			let (m,p) = (n+1,(u,v,d));
			let r = ds.entry(p).or_insert(usize::MAX);
			if m < *r { *r = m; qu.push(Reverse((m,p))); }
		}
		let (m,p) = (n+1000,(x,y,d^1));
		let r = ds.entry(p).or_insert(usize::MAX);
		if m < *r { *r = m; qu.push(Reverse((m,p))); }
		let (m,p) = (n+1000,(x,y,d^3));
		let r = ds.entry(p).or_insert(usize::MAX);
		if m < *r { *r = m; qu.push(Reverse((m,p))); }
		let (m,p) = (n+2000,(x,y,d^2));
		let r = ds.entry(p).or_insert(usize::MAX);
		if m < *r { *r = m; qu.push(Reverse((m,p))); }
	}

	let (ex,ey) = (wd-2,1);
	let ed:Vec<_> = ds.keys().filter(|&&(x,y,_)| x==ex && y==ey).map(|&p| (*ds.get(&p).unwrap(),p)).collect();
	let z = ed.iter().map(|&(n,_)| n).min().unwrap();
	let mut vz = HashSet::new();
	ed.iter().filter(|&&(n,_)| n == z).for_each(|&(n,p)| find(n,p,&ds,&mut vz));
	let z2 = HashSet::<(i16,i16)>::from_iter(vz.iter().map(|&(x,y,_)| (x,y))).len();
	(z.to_string(),z2.to_string())
}

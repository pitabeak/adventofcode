use std::io::BufRead;

#[derive(Debug,Clone)]
struct Monkey {
	it:Vec<i32>,
	op:i32,
	dv:i32,
	tr:usize,
	fa:usize,
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut it = f.lines().map(|s| s.unwrap());
	let mut da = Vec::new();
	loop {
		it.next();
		let s = it.next().unwrap();
		let s = s.rsplit_once(": ").unwrap().1;
		let a:Vec<i32> = s.split(", ").map(|s| s.parse().unwrap()).collect();
		let s = it.next().unwrap();
		let o:i32 = if s.ends_with("old * old") {
			0
		} else if let Some((_,u)) = s.rsplit_once("+ ") {
			u.parse().unwrap()
		} else {
			let u = s.rsplit_once("* ").unwrap().1;
			-u.parse::<i32>().unwrap()
		};
		let v:Vec<i32> = (0..3).map(|_| it.next().unwrap().rsplit_once(' ').unwrap().1.parse().unwrap()).collect();
		da.push(Monkey{it:a,op:o,dv:v[0],tr:v[1] as usize,fa:v[2] as usize});
		if it.next().is_none() { break; }
	}
	let mut d2 = da.clone();

	let mut ct = vec![0;da.len()];
	for _ in 0..20 {
		for i in 0..da.len() {
			for _ in 0..da[i].it.len() {
				ct[i] += 1;
				let mut x = da[i].it.remove(0);
				x = if da[i].op == 0 {
						x*x
					} else if da[i].op > 0 {
						x+da[i].op
					} else {
						x*-da[i].op
					};
				x /= 3;
				let j = if x%da[i].dv == 0 {da[i].tr} else {da[i].fa};
				da[j].it.push(x);
			}
		}
	}
	ct.sort_by(|a,b| b.cmp(a));
	let z = ct[0]*ct[1];

	let mut da = d2;
	let md:i32 = da.iter().map(|m| m.dv).product();
	let mut ct = vec![0;da.len()];
	for _ in 0..10_000 {
		for i in 0..da.len() {
			for _ in 0..da[i].it.len() {
				ct[i] += 1;
				let mut x = da[i].it.remove(0);
				x = if da[i].op == 0 {
						((x as i64).pow(2)%(md as i64)) as i32
					} else if da[i].op > 0 {
						(x+da[i].op)%md
					} else {
						(x*-da[i].op)%md
					};
				let j = if x%da[i].dv == 0 {da[i].tr} else {da[i].fa};
				da[j].it.push(x);
			}
		}
	}
	ct.sort_by(|a,b| b.cmp(a));
	let z2 = (ct[0] as i64)*(ct[1] as i64);

	(z.to_string(),z2.to_string())
}

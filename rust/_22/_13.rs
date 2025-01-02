use std::io::BufRead;
use serde_json::Value;
use std::cmp::Ordering;

fn compare(a:&Value,b:&Value) -> Ordering {
	match a {
		Value::Number(x) => {
			match b {
				Value::Number(y) => x.as_u64().cmp(&y.as_u64()),
				Value::Array(_) => compare(&Value::Array(vec![a.clone()]),b),
				_ => panic!()
			}
		}
		Value::Array(p) => {
			match b {
				Value::Number(_) => compare(a,&Value::Array(vec![b.clone()])),
				Value::Array(q) => {
					for i in 0..p.len().min(q.len()) {
						let z = compare(&p[i],&q[i]);
						if z.is_ne() { return z; }
					}
					p.len().cmp(&q.len())
				}
				_ => panic!()
			}
		}
		_ => panic!()
	}
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut z = 0;
	let mut a = Vec::new();
	for s in f.lines() {
		let s = s.unwrap();
		if !s.is_empty() {
			a.push(serde_json::from_str::<Value>(&s).unwrap());
			let i = a.len();
			if i%2==0 && compare(&a[i-2],&a[i-1]).is_lt() {
				z += i/2;
			}
		}
	}
	let x = serde_json::from_str::<Value>("[[2]]").unwrap();
	let y = serde_json::from_str::<Value>("[[6]]").unwrap();
	a.push(x.clone());
	a.push(y.clone());
	a.sort_by(|a,b| compare(&a,&b));
	let z2 = (a.iter().position(|t| t==&x).unwrap() + 1) * (a.iter().position(|t| t==&y).unwrap() + 1);
	(z.to_string(),z2.to_string())
}

/*
use std::io::BufRead;
use std::fmt;
use std::cmp::{Ordering,Ord,PartialOrd,Eq,PartialEq};

#[derive(Clone)]
enum Value {
	Number(u8),
	List(Vec<Value>)
}

impl fmt::Debug for Value {
	fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Value::Number(x) => write!(f,"{x}"),
			Value::List(a) => {
				if a.is_empty() {
					write!(f,"[]")
				} else {
					write!(f,"[{:?}",a[0]).unwrap();
					for x in a.iter().skip(1) {
						write!(f,",{x:?}").unwrap();
					}
					write!(f,"]")
				}
			}
		}
	}
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match self {
			Value::Number(x) => {
				match other {
					Value::Number(y) => x.cmp(y),
					Value::List(_) => Value::List(vec![self.clone()]).cmp(other)
				}
			}
			Value::List(a) => {
				match other {
					Value::Number(_) => self.cmp(&Value::List(vec![other.clone()])),
					Value::List(b) => {
						for i in 0..a.len().min(b.len()) {
							let z = a[i].cmp(&b[i]);
							if z.is_ne() { return z; }
						}
						a.len().cmp(&b.len())
					}
				}
			}
		}
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
		self.cmp(other).is_eq()
    }
}

impl Eq for Value {}

fn read_value(s:&str) -> Value {
	let mut it = s.chars();
	let mut c = it.next();
	let mut st = Vec::new();
	loop {
		let v =
		if Some('[') == c {
			c = it.next();
			if Some(']') == c {} else { st.push(Vec::new()); continue; }
			c = it.next();
			Value::List(Vec::new())
		} else {
			let mut x = 0;
			while let Some(d) = c {
				let d = d as u8;
				if !(b'0'<=d && d<=b'9') { break; }
				x = x*10 + (d - b'0');
				c = it.next();
			}
			Value::Number(x)
		};
		if st.is_empty() { return v; }
		let l = st.len();
		st[l-1].push(v);
		while Some(']') == c {
			c = it.next();
			let v = Value::List(st.pop().unwrap());
			if st.is_empty() { return v; }
			let l = st.len();
			st[l-1].push(v);
		}
		assert!(Some(',') == c);
		c = it.next();
	}
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut z = 0;
	let mut a = Vec::new();
	for s in f.lines() {
		let s = s.unwrap();
		if !s.is_empty() {
			a.push(read_value(&s));
			let i = a.len();
			if i%2==0 && a[i-2] < a[i-1] {
				z += i/2;
			}
		}
	}
	let x = read_value("[[2]]");
	let y = read_value("[[6]]");
	a.push(x.clone());
	a.push(y.clone());
	a.sort();
	let z2 = (a.iter().position(|t| t==&x).unwrap() + 1) * (a.iter().position(|t| t==&y).unwrap() + 1);
	(z.to_string(),z2.to_string())
}
*/

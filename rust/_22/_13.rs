use std::io::BufRead;
use std::fmt;

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
					write!(f,"[{:?}",a[0]);
					for x in a.iter().skip(1) {
						write!(f,",{x:?}");
					}
					write!(f,"]")
				}
			}
		}
	}
}

fn read_value(s:&str) -> Value {
	let mut it = s.chars();
	let mut c = it.next();
	let mut st = Vec::new();
	loop {
		let mut v =
		if Some('[') == c {
			c = it.next();
			if Some(']') == c {} else { st.push(Vec::new()); continue; }
			c = it.next();
			Value::List(Vec::new())
		} else {
			let mut x = 0;
			while let Some(d) = c {
				x = x*10 + d::parse<u8().unwrap();
				if !(b'0'<=d && d<=b'9') { break; }
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
		assert!(Some(b',') == c);
		c = it.next();
	}
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	for s in f.lines() {
		let s = s.unwrap();
		if s.len() > 0 { println!("{:?}",read_value(&s.as_bytes())); }
	}
	(0.to_string(),0.to_string())
}

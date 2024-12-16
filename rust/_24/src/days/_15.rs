use std::io::BufRead;
use std::str::from_utf8;

fn can_push(x:usize, y:usize, dy:i32, da:&Vec<Vec<u8>>) -> bool {
	let q = (y as i32 + dy) as usize;
	match da[q][x] {
		b'#' => return false,
		b'[' => return can_push(x,q,dy,da),
		b']' => {
			if !can_push(x-1,q,dy,da) { return false; }
		}
		b'.' => {}
		_ => panic!()
	}
	match da[q][x+1] {
		b'#' => false,
		b'.' => true,
		b'[' => can_push(x+1,q,dy,da),
		_ => panic!()
	}
}

fn do_push(x:usize, y:usize, dy:i32, da:&mut Vec<Vec<u8>>) {
	let q = (y as i32 + dy) as usize;
	match da[q][x] {
		b'[' => do_push(x,q,dy,da),
		b']' => do_push(x-1,q,dy,da),
		b'.' => {}
		_ => panic!()
	}
	match da[q][x+1] {
		b'[' => do_push(x+1,q,dy,da),
		b'.' => {}
		_ => panic!()
	}
	da[q][x] = b'[';
	da[q][x+1] = b']';
	da[y][x] = b'.';
	da[y][x+1] = b'.';
}

pub fn solve(f:Box<dyn BufRead>) -> (String,String) {
	let mut it = f.lines().map(|i| i.unwrap());
	let mut da = Vec::new();
	let mut d1 = Vec::new();
	let mut j = String::new();
	loop {
		let i = it.next().unwrap();
		if i.len() == 0 { break; }
		let i = i.as_bytes().to_vec();
		j.clear();
		j.extend(i.iter().map(|x| match x { b'#'=>"##", b'O'=>"[]",
			b'.'=>"..", b'@'=>"@.", _=>panic!() }));
		da.push(i);
		d1.push(j.as_bytes().to_vec());
	}
	let wd = da[0].len();
	let ht = da.len();
	let (gx,gy) = (0..wd*ht).map(|i| (i/ht,i%ht)).filter(|&(x,y)| da[y][x]==b'@').next().unwrap();
	let mut x = gx;
	let mut y = gy;
	let mut x1 = 2*gx;
	let mut y1 = gy;
	while let Some(i) = it.next() {
		for j in i.bytes() {
			let (dx,dy) = match j { b'^'=>(0,-1), b'v'=>(0,1), b'<'=>(-1,0), b'>'=>(1,0), _=>panic!() };
			let mut p = x;
			let mut q = y;
			loop {
				p = (p as i32 + dx) as usize;
				q = (q as i32 + dy) as usize;
				match da[q][p] {
					b'.' => {
						da[q][p] = b'O';
						da[y][x] = b'.';
						x = (x as i32 + dx) as usize;
						y = (y as i32 + dy) as usize;
						da[y][x] = b'@';
						break;
					}
					b'#' => break,
					b'O' => {}
					_ => panic!()
				}
			}
			if dy == 0 {
				let mut p = x1;
				loop {
					p = (p as i32 + dx) as usize;
					match d1[y1][p] {
						b'.' => {
							while p != x1 {
								let r = (p as i32 - dx) as usize;
								d1[y1][p] = d1[y1][r];
								p = r;
							}
							d1[y1][x1] = b'.';
							x1 = (x1 as i32 + dx) as usize;
							break;
						}
						b'#' => break,
						b'['|b']' => {}
						_ => panic!()
					}
				}
			} else {
				let q = (y1 as i32 + dy) as usize;
				let mut f = true;
				match d1[q][x1] {
					b'.' => {}
					b'#' => f = false,
					b'[' => {
						if can_push(x1,q,dy,&d1) {
							do_push(x1,q,dy,&mut d1);
						} else {
							f = false;
						}
					}
					b']' => {
						if can_push(x1-1,q,dy,&d1) {
							do_push(x1-1,q,dy,&mut d1);
						} else {
							f = false;
						}
					}
					_ => panic!()
				}
				if f {
					d1[y1][x1] = b'.';
					y1 = q;
					d1[y1][x1] = b'@';
				}
			}
		}
	}
/*
	for i in 0..ht {
		println!("{}",from_utf8(&d1[i]).unwrap());
	}
*/
	let z:usize = (0..wd*ht).map(|i| (i/ht,i%ht)).filter_map(|(x,y)| (da[y][x] == b'O').then(|| y*100+x)).sum();
	let z2:usize = (0..wd*2*ht).map(|i| (i/ht,i%ht)).filter_map(|(x,y)| (d1[y][x] == b'[').then(|| y*100+x)).sum();
	(z.to_string(),z2.to_string())
}

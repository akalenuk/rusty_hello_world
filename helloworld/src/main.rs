fn main() {
	let xs = [1f64, 2f64, 3f64];
	let is = [1i32, 2i32, 3i32];
	for i in 0..xs.len() {
		println!("{}", xs[i]);}
	for x in &xs {
		println!("{}", x);}
	for x in xs.iter() {
		println!("{}", x);}
	for i in 1..(3+1) {
		let x = i as f64;
		println!("{}", x);}
	for i in &is {
		println!("{}", i);}
	for i in &is {
		let x = *i as f64;
		println!("{}", x);}
	for x in &xs {
		printf64(*x);}
	for i in &is {
		printi64(*i as i64);}
}

fn printf64(x:f64) {
	println!("{}", x);}

fn printi64(x:i64) {
	let y = x as f64;
	println!("{}", y);}


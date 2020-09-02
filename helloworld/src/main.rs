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
}

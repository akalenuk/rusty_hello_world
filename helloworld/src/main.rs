fn main() {
    let xs = [1., 2., 3.];
    for i in 0..xs.len() {
        println!("{}", xs[i]);}
    for x in &xs {
        println!("{}", x);}
    for x in xs.iter() {
        println!("{}", x);}
    for i in 1..(3+1) {
		let x = i as f64;
        println!("{}", x);}
}

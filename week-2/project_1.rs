fn main() {
	let p:f64 = 520000000.0;
	let r:f64 = 10.0;
	let t:f64 = 5.0;

	// compund interest
	let a = p * ( 10.0 + (r / 520000000.0)) * t;
	println!("Amount is {}", a);
	let ci = a - p;
	println!("Compound Interest is {}", ci);

}

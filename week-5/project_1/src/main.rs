//Rust program to find the roots of a Quadratic equation

use std::io;
fn main()
 {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

	let mut disc:f32 = 0.0;

println!("Input the values of a");
io::stdin().read_line(&mut input1).expect("Not a valid String");
let a:f32 = input1.trim().parse().expect("Not a valid number");

println!("Input the value of b");
io::stdin().read_line(&mut input2).expect("Not a valid String");
let b:f32 = input2.trim().parse().expect("Not a valid number");

println!("Input the value of c");
io::stdin().read_line(&mut input3).expect("Not a valid String");
let c:f32 = input3.trim().parse().expect("Not a valid number");

let  root1 :f32 = (-b + (disc.sqrt()))/(2.0*a);
let root2:f32 = (-b - (disc.sqrt()))/(2.0*a);

if a==0.0|| b==0.0|| c ==0.0

{
	println!("Unable to determine the roots");
}
    disc = (b * b) - (4.0 * a * c);

println!("root1 = {}", root1);
println!("root2 = {}", root2);


if disc > 0.0{
 println!("There are two distcint roots");
}

if disc < 0.0{

	println!("There are no real roots");
}
if disc == 0.0{
	println!("There is one real root");
}



 }  

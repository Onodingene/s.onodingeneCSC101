use std::io;
fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();

let employee = String::new();
let experienced = String::new();
let inexperienced = String::new();
  let mut age:i32 = 0;

  
	println!("Are you An Experienced Employee or Inexperienced Employee") ;
   io::stdin().read_line(&mut input2).expect("Not a valid String");
  

println!("How old are you?");

let mut input1 = String::new();
io::stdin().read_line(&mut input1).expect("Failed to read string");
let age:i32 = input1.trim().parse().expect("Failed to input");
	
	//io::stdin().read_line(&mut age).expect("Not a valid string");

  //io::stdin().read_line(&mut age).expect("Not a valid string");
//	let age:i32 = age.trim().parse().expect("Not a valid Number");


  if age >= 40 
  {
println!("Your incentive is N1,560,000 ");
  } else if  age >= 30 && age < 40  
  {
  println!("Your incentive is N1,480,00.00");

  } else if age < 28 && employee == experienced
  {
  println!("Your incentive is N1,300,000 per month");

  } else if employee == "inexperienced" 
 { 
  println!("Your incentive is N100,000");
 } else {
   println!("Not a situation");
 }
 }
//print!("{}{}",age,employee );



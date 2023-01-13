// Rust program that displays the following menu of food items for a customer to take its order
use std::io;
fn main() {


	
println!("\n   Menu\np = Poundo Yam / Edikaiko Soup    =  ₦3,200\nF = Fried Rice and chicken        =  ₦3,000\nA = Amala & Ewedu Soup            =  ₦2,500\nE = Eba & Ewedu Soup              =  ₦2,000\nW = White Rice & Stew             =  ₦2,500");

println!("/n");

println!("\n Press 1 = Poundo Yam / Edikaiko Soup    =  ₦3,200\n Press 2 = Fried Rice and chicken        =  ₦3,000\n Press 3 = Amala & Ewedu Soup      =  ₦2,500\n Press 4 = Eba & Ewedu Soup         =  ₦2,000 \n Press 5 = White rice and stew = ₦2,500"  );
 
 order();

 let mut input3 = String::new();
println!("Do you want anything else ? ");
io::stdin().read_line(&mut input3).expect("Not a valid String");

 
 while input3   == "Yes"{
order(); 
println!("Thank you! Your order has been taken.");
}
}

fn order() {
let mut input1 = String::new();
println! ("Make your choice of food: ");
io::stdin().read_line(&mut input1).expect("Not a valid string");
let input1:i32 = input1.trim().parse().expect("Not a Valid Integer");
println!("Food choice is:{}",input1);

if input1 == 1 {
	println!(" Price  =  ₦3,200");
} else if input1 == 2 {
	println!("Fried Rice and chicken = ₦3,000");
} else if input1 == 3 {
	println!("Price     =  ₦2,500");
} else if input1 == 4 {
	println!("Price       =  ₦2,000");
} else if input1  == 5 {
	println!("Price =  ₦ 2,500");
} else{
	println!("Not Computable");
} 

let quantity:i32 = 0;
let mut input2 = String::new();  
	println!("Input the quantity of food you want");
	io::stdin().read_line(&mut  input2).expect("Not a valid String");
let q:i32 = input2.trim().parse().expect("Not a valid integer");

let mut price = 3200;
price = 3000;
price = 2500;
price = 2000;
price = 2500;


let mut quantity:i32 = 0;
let amount = quantity * price;


/*let mut input3 = String::new();
println!("Do you want anything else ? ");
io::stdin().read_line(&mut input3).expect("Not a valid String");

 println!("Thank you! Your order has been taken.");*/
}

	





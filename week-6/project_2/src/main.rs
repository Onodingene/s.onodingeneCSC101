// program to find out the information of siblings
use std::io;


fn main() { 
//create a for or while loop to ask how many siblings they have 
let mut input1 = String::new();
println!("What is the first name of your sibling? ");
io::stdin().read_line(&mut input1).expect("Not a valid Input");
println!("The name of my sibling is {}", input1);


//let mut input3 = String::new();
//io::stdin().read_line(&mut input3).expect("Not a valid Input");
//let b:i32 = input3.trim().parse().expect("Not a valid integer");
//println!("\n");

let mut input2 = String::new();
println!("How old is your sibling? ");
io::stdin().read_line(&mut input2).expect("Not a valid input");
let a:i32 = input2.trim().parse().expect("Not a valid integer");
println!("My sibling is {} years old", input2);

let mut input3 = String::new();
println!("Are you married or single?");
io::stdin().read_line(&mut input3).expect("Not a valid Input");
let b:i32 = input3.trim().parse().expect("Not a valid integer");
println!("\n");

if a >18{
	println!("Are you married or single?");
	println!("\n");


	println!("\nPress 1 if you are married\n Press 2 if you are single");
	if b == 2 {
		println!("Are you a student or a worker?");
		println!("Press 1 if you are a student\n Press 2 if you are a worker");
			if b == 1{
			println!("Which University are you in?");
			let mut input4 = String::new();
			io::stdin().read_line(&mut input4).expect("Not a valid input");
			println!("I am in {}", input4);
			}
			let mut input5 = String::new();
			io::stdin().read_line(&mut input5).expect("Not a valid input");
			println!("What course are you studying?");
			println!("I am studying {}", input5);
			}else {
				println!("You are a great worker!");
			}
	}else if b == 1 {
		println!("Do you have any offsprings?");
		println!("Press 1 if you have an offspring and Press 2 if you DONOT  have any offspring");

		let mut input6 = String::new();
		println!("Which city does the offspring live in?");
		io::stdin().read_line(&mut input6).expect("Not a valid input");

	
} else {
	println!("Have you written WAEC?");
	let mut input7 = String::new();
	io::stdin().read_line(&mut input7).expect("Not a valid input");
	println!("Press 1 if you have written WAEC and press 2 if you have not written WAEC");
}
let mut input11 = String::new();
io::stdin().read_line(&mut input11).expect("Not a valid Input");
let c:i32 = input11.trim().parse().expect("Not a valid integer");
	if c == 1 {
		println!("Which secondary school did you attend?");
		let mut input8 = String::new();
		io::stdin().read_line(&mut input8).expect("Not a valid Input");
		println!("The name of my secondary school is {}", input8);
	}else {
		println!("What class are you in currently?");
		let mut input9 = String::new();
		io::stdin().read_line(&mut input9).expect("Not a valid input");
	}


	
}



// Rust prgram to select an equation, read the inputs and perform the corresponding calculations 
 use std::io;
 fn main() {
 let mut altitude = String::new();
 let mut length_of_side = String::new();
 let mut radius = String::new();
 let mut diagonal1 = String::new();
 let mut diagonal2 = String::new();


 

 println!("Area of Trapezium formula = Height/2( base1 + base2)
 	       Area of the rhombus formula = 1/2 * diagonal1 * diagonal2
 	       Area of parallelogram formula = base * altitude
 	       Area of cube formula = 6 * (length of the side) ^ 2
 	       Volume of Cylinder Formula = Pi * radius ^ 2 * height");
 println!("\n");
 println!("-----------------------------------------------------------------------");
 println!("\n");


println!("Press 1 = Area of Trapezium formula = Height/2( base1 + base2)\nPress 2 = Area of the rhombus formula = 1/2 * diagonal1 * diagonal\nPress 3 = Area of parallelogram formula = base * altitude\nPress 4 = Area of cube formula = 6 * (length of the side) ^ 2\nPress 5 = Volume of Cylinder Formula = Pi * radius ^ 2 * height");

let mut input1 = String::new();
 println!("Choose The formula to work with");
 io::stdin().read_line(&mut input1).expect("Failed to read input");
 let input1:i32 = input1.trim().parse().expect("Not a valid integer");


 if input1 == 1{
 	println!("Calculating the area of a trapezium");
 	trapezium();
} else if input1 == 2{
	println!("Calculating the area of a rhombus");
	rhombus();
} else if input1 == 3{
	println!("Calculating the area of a Parallelogram");
	parallelogram();
} else if input1 == 4{
	println!("Calculating the area of a Cube");
	cube();
} else if input1 == 5{
	println!("Calculating the volme of a Cylinder");
	cylinder();
} else 
{
	println!("Not computable");
}



fn trapezium(){
// Area of Trapezium formula = Height/2( base1 + base2)
 let mut input2 = String::new();
 println!("Input the base1 of the trapezium");
 io::stdin().read_line(&mut input2).expect("Failed to read input");
 let a:f32 = input2.trim().parse().expect("Not a valid integer");

let mut input3 = String::new();
 println!("Input the base2 of the trapezium");
 io::stdin().read_line(&mut input3).expect("Failed to read input");
 let b:f32 = input3.trim().parse().expect("Not a valid integer");

let mut input4 = String::new();
 println!("Input the height of the trapezium");
 io::stdin().read_line(&mut input4).expect("Failed to read input");
 let c:f32 = input4.trim().parse().expect("Not a valid integer");
let area:f32 = (( a + b)/ 2.0)* c;

println!("The area of the trapezium is: {}", area);
}


fn rhombus(){
 // Area of the rhombus formula = 1/2 * diagonal1 * diagonal2
 let mut input5 = String::new();
 println!("Input diagonal1 of the rhombus");
 io::stdin().read_line(&mut input5).expect("Failed to read input");
 let d:f32 = input5.trim().parse().expect("Not a valid integer");

let mut input6 = String::new();
 println!("Input diagonal2 of the rhombus");
 io::stdin().read_line(&mut input6).expect("Failed to read input");
 let e:f32 = input6.trim().parse().expect("Not a valid integer");
let area:f32 = 1.0/2.0 * (d * e);

println!("The area of the rhombus is: {}", area);
}

fn parallelogram(){
// Area of the parallelogram = base * altitude
let mut input7 = String::new();
println!("Input the base of the parallelogram");
io::stdin().read_line(&mut input7).expect("Failed ot read input");
 let f:f32 = input7.trim().parse().expect("Not a valid integer");

let mut input8 = String::new();
 println!("Input the altitude of the parallelogram");
 io::stdin().read_line(&mut input8).expect("Failed to read input");
 let g:f32 = input8.trim().parse().expect("Not a valid integer");
let area: f32 = f * g;
println!("The area of the parallelogram is {}", area);
}


fn cube(){
// Area of a cube = 6 * (length of the side) ^ 2
let mut input9 = String::new();
println!("Calculating the area of a cube");
println!("Input the length of the side of the cube");
io::stdin().read_line(&mut input9).expect("Failed to read input");
 let h:f32 = input9.trim().parse().expect("Not a valid integer");
let area = 6 * (h as i32 ^ 2);
println!("The area of the cube is {}", area);
}


fn cylinder(){
// Volume of Cylinder formula = Pi * radius ^ 2 * height
let mut input10 = String::new();
println!("Calculating the volume of a cylinder");
println!("Input the radius of the cylinder");
 io::stdin().read_line(&mut input10).expect("Failed to read input");
 let i:f32 = input10.trim().parse().expect("Not a valid integer");

 println!("Input the height of the cylinder");
 let mut input11 = String::new();
 io::stdin().read_line(&mut input11).expect("Failed to read input");
 let j:i32 = input11.trim().parse().expect("Not a valid integer");
 let volume:i32 = ((314/100) * (i as i32  ^ 2)) * j as i32;
 println!("The Volume of the cylinder is {}", volume);
}
}






















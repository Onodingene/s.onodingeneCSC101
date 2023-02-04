use std::io;
use std::io::Read;

fn main() {
    println!("WELCOME TO GLOBACOM LIMITED AND CO!");
    println!("\n");

    let mut input1 = String::new();
    println!("What department do you belong to?");
    println!("Press 1 = Administaror
    	      Press 2 = Project Manager
    	      Press 3 = Employee
    	      press 4 = Customer
    	      Press 5 = Vendor" );
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input1.trim().parse().expect("Not a valid Integer");

    if a == 1 {
    	administrator();
    }else if a == 2{
    	project();
    }else if a == 3{
    	employee();
    }else if a == 4{
    	customer();
    }else if a == 5{
    	vendor();
    }else {
    	print!("Not computable!");
    }

}

fn administrator(){
	println!("Welcome fellow Administaror, you can sucessfully view the globacom databse!");
	let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}", contents);

	println!("{}", "globacom");

}

fn project(){
	println!("Welcome fellow Project Manager, you can sucessfully view the Project table!");
	let mut file = std::fs::File::open("project_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}", contents);

	println!("{}", "project");
}

fn employee(){
	println!("Welcome fellow Employee, you can sucessfully view the Staff table!");
	let mut file = std::fs::File::open("staff_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}", contents);

	println!("{}", "staff");
}

fn customer(){
	println!("Welcome fellow Customer, you can sucessfully view the Customer table!");
	let mut file = std::fs::File::open("staff_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}", contents);

	println!("{}", "customer");
}

fn vendor(){
	println!("Welcome fellow Customer, you can sucessfully view the Dataplan table!");
	let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}", contents);

	println!("{}", "dataplan");
}



use std::io;
fn main() {
	GeoPo_Merger();
	Pub_Service();
  

    let mut input1 = String::new();
    println!("Which division of service do you belong to?(Enter 1 = Geopolitical Zoning, Enter 0 = Public Servic Checker ");
    io::stdin().read_line(&mut input1).expect("Not a valid integer");



    fn GeoPo_Merger(){
    	//carry out the loop here or check my previou work
    	let mut input2 = String::new();
    	println!("What is your name?(\nEnter 1 = Aigbogun Alamba Daudu,\n Enter 2 = Murtala Afees Bendu,\n Enter 3 = Okorocha Calistus Ogbona , \nEnter 4 = Adewale Jimoh Akanabi, \nEnter 5 = Osazuwa Faith Etieye");
    	io::stdin().read_line(&mut input2).expect("Not a valid String");
    	println!("Your name is {}",input2);

    	let mut input3 = String::new();
    	println!("What ministry do you belong to?(\nEnter 1 = Internal Affairs, \nEnter 2 = Justice, \nEnter 3 = Defence, \nEnter 4 = Power and Steel, \nEnter 5 = Petroleum");
    	io::stdin().read_line(&mut input3).expect("Not a valid String");
    	println!("Your ministry is {}", input3);


    	let mut input4 = String::new();
    	println!("What geopolitical Zone do you belong to?(Enter 1 = South West, Enter 2 = North East,\n Enter 3 = South South , \nEnter 4 = South West, \nEnter 5 = South East");
    	io::stdin().read_line(&mut input4).expect("Not a valid String");
    	println!("Your geopolitical zone is {}", input4);


}


fn Pub_Service(){
	loop{
		let public servant = [Office administaror, Academic, Lawyer, Teacher]
	let office administarors = [Intern, Administrator, Senior Administrator, Office Manager, Director, ceo]
	let Academic = [Research Assistant, Phd Candidate, PostDoc Researcher, Senior Lecturer, Dean]
	let Lawyer = [Paralegal, Junio Associate, Associate, Senior Associate 1 2, Senior Associate 3 4, Partner ]
	let Teacher = [Placement, Classroom, teacher, Snr Teacher, Leading Teacher, Deputy Teacher, Principal]

	let mut input5 = String::new();
	println!("How many years of work experience do you have?");
	if input5 == 1 || input5 == 2 && Public servant == Office administrator
	for a in office adm.iter(){
	println!("The Office adminstrator is a {}", a);
} else if input5 == 3 || input5 == 4 || input5 = 5 && public servant == Academic
for b in acad.iter(){
println!("The acadmic is a {}", b);
}else if input5 == 5 || input5 == 6 || input5 == 7 || input5 == 8 && public servant == lawyer
for c in in law.iter(){
	println!("The Lawyer is a {}", c)
}else if input5 == 8 || input5 ==9 || input5 == 10 && public servant = Teacher
for d in teach.iter(){
	println!("The teacher is a {}", d);
}else if input5 == 11 || input5 == 12 || input5 == 13 && public servant = Office administartor == 

	}else if input5 > 14 
	}else 	println!("Not computable");
	}






    


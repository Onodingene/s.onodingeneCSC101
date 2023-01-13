
use std::io::write;
use std::io;

fn code_7() {
println!("Welcome, Code 7 staff's!");
let mut input2 = String::new();
println!("Press 1 if you want to open the file for Aigbona Juliet
	      Press 2 if you want to open the file for Akpevwe Iloka");
io::stdin().read_line(&mut input2).expect("Not a valid input");
let k:i32 = input1.trim().parse().expect("Not a valid integer");

if k == 1{
	let mut file = std::fs::File::create("aigbona_juliet.txt").expect("create failed");
	let consulting = vec!["Analytics Consulting Services", "Customer Experience","cyber security,strategy, risk, compliance and resilence","digital transformation", "risk consulting services","Supply chain and operations","Technology transformation"];
 	for a in consulting.iter(){
	file.write_all("\nName: Aigbona Juliet".as_bytes()).expect("write failed");
	file.write_all("\nQualification: B.Sc".as_bytes()).expect("write failed");
	file.write_all("\nDepartment: Consulting".as_bytes()).expect("write failed");
	file.write_all(consulting.as_bytes()).expect("write failed");
	print!("{}",a);
	}
}else {	
	let mut file = std::fs::File::create("Akpevwe_Iloka.txt").expect("create failed");
	let assurance = vec!["Audit service","Climate change and sustainability services", "Financial accounting advisory services","Forensic and integrity services", "Private client audit experience", "Accounting link","Assurance"];	
	for b in assurance.iter(){
	file.write_all("\nName: Akpevwe Iloka".as_bytes()).expect("write failed");
	file.write_all("\nQualification: HND".as_bytes()).expect("write failed");
	file.write_all("\nDepartment: Assurance".as_bytes()).expect("write failed");
	file.write_all(assurance.as_bytes()).expect("write failed");
	print!("{}",b);
	}
}
}

fn code_9() {
	println!("Welcome, Code 9 Staff's!");
	let mut input3 = String::new();
	println!("Press 1 if you want to open the file for Ehis Ero
		      Press 2 if you want to open the file for Maria Akinsola");
	io::stdin().read_line(&mut input3).expect("Not a valid input");
	let i:i32 = input3.trim().parse().expect("Not a valid integer");

	if i == 1 {
	let mut file = std::fs::File::create("Ehis_Ero.txt").expect("create failed");	
	let strategy = vec!["Strategy consulting", "Customer Experience", "Transcation strategy and execution", "Restructuring and turnaround strategy", "Industry strategy","Digital business building","Comercial strategy"];
	for c in strategy.iter(){
	file.write_all("\nName: Ehis Ero".as_bytes()).expect("Write failed");
	file.write_all("\nQualification: M.Sc".as_bytes()).expect("Write failed");
	file.write_all("\nDepartment: Strategy".as_bytes()).expect("Write failed");
	file.write_all(strategy.as_bytes()).expect("Write failed");
	print!("{}",c);
	}
	} else {
	let mut file = std::fs::File::create("Maria_Akinsola.txt").expect("create failed");		
	let transactions =vec!["Corporate finance","Divestements and carve-outs","sustainability and ESG services","M & A Integration", "M & A advisory", "M & A technology and tools", "M & A advanced analytics"];
	for d in transactions.iter(){
	file.write_all("\nName: Maria_Akinsola".as_bytes()).expect("Write failed");
	file.write_all("\nQualification: M.Sc".as_bytes()).expect("write failed");
	file.write_all("\n Department: Transactions and corporate finance".as_bytes()).expect("Write failed");
	file.write_all(transactions.as_bytes()).expect("Write failed");
	print!("{}",d);
	}
 }

fn code_8(){
	println!("Welcome, code 8 Staff's!");
	let mut input4 = String::new();
	println!("Press 1 if you want to open the file for Adamu Sagamu
		      Press 2 if you want to open the file for Gbenga Daniels");
	io::stdin.read_line(&mut input4).expect("Not a valid input");
	let j:i32 = input4.trim().parse().expect("Not a valid integer");

    if j == 1{
	let tax = vec!["Tax planning", "Tax function operations", "Tax policy and controversy", "Global trade", "Tax accounting", "Tax complaince", "Transcation tax"];
	for e in tax.iter(){
	file.write_all("\nName: Adamu Sagamu".as_bytes()).expect("Write failed");
	file.write_all("\nQualification: B.Sc".as_bytes()).expect("write failed");
	file.write_all("\n Department: Tax".as_bytes()).expect("Write failed");
	file.write_all(tax.as_bytes()).expect("Write failed");
    print!("{}",e);
	}
    }else {
	let workforce = vec!["Change management and Experience", "HR transformation", "Integrated workforce mobility", "learning and development consulting", "Recognition and reward avdisory","Workforce analytics","People and workforce"];
	for f in workforce.iter(){
	file.write_all("\nName: Gbenga Daniels".as_bytes()).expect("Write failed");
	file.write_all("\nQualification: HND".as_bytes()).expect("write failed");
	file.write_all("\n Department: People and Workforce".as_bytes()).expect("Write failed");
	file.write_all(tax.as_bytes()).expect("Write failed");
	print!("{}",f);
	}
    }
}

fn main(){
	let mut file = std::fs::File::create("holiday_projecy.txt").expect("create failed");
   file.write_all("\nWELCOME TO ERNST & YOUNG GLOBAL LIMITED!".as_bytes()).expect("Write failed");
println!("\nData written to file.");

	Let mut input1 = String::new();
	println!("What is your department code?")
	io::stdin().read_line(&mut input1).expect("Not a valid input");
	let g:i32 = input1.trim().parse().expect("Not a valid integer");
if g == 7{
	code_7();
}else if g == 8{
	code_8();
}else if g ==9{
	code_9();
}else {
	println!("Not computab;e ")
}		
}
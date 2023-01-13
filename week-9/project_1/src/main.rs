use std::io::Write;
use std::fs::File;

fn main() {

	let student_name = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolande", "Adekunle Gold", "Blanca Edemoh"];
	let matric_number = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202","MEE10202001"];
	let department = vec!["Accounting", "Economics","Computer", "Electrical", "Mechanical"];
	let level = vec!["300", "100", "200", "200", "100"];

	print!("\nStudent name\nMatric Number\nDepartment\nLevel");

let mut file = std::fs::file::create("PAU-SIMS").expect("Create failed");

for i in 0..matric_number.len()
	
 file.write_all("matric_number [i]".as_bytes()).expect("Write failed");
 file.write_all(""as_bytes()).expect("write failed");
 file.write_all("stdent_name [i]".as_bytes()).expect("Write failed");
 file.write_all(""as_bytes()).expect("write failed");
 file.write_all("department [i]".as_bytes()).expect("Write failed");
 file.write_all(""as_bytes()).expect("write failed");
 file.write_all("level [i]".as_bytes()).expect("Write failed");
 file.write_all(""as_bytes()).expect("write failed");
 print!("\nStdent name: {},\nMatric Number:{}\n,\nDepartmet{},\nLevel:{}", student_name, matric_number, department,level);
}

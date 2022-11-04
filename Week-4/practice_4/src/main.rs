fn main() {
    let fullname = "Chibudum Jon Umeh";
    let department = "Computer Science";
    let uni = "Pan-Atlantiv University";

    let mut school = "School of Science".to_string();
    // push string
    school.push_str(" and Technology");

    println!("My name is {}", fullname);
    //check lenght
    println!("The length my fullname is: {}",fullname.len());
    println!("I am a student of {} Department", department);
    println!("{}",school);
    println!("{}",uni)
}

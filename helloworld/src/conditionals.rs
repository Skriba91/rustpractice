pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age = true;


    // If else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bruh, you can drink!");
    } else if age < 21 && check_id {
        println!("Bruh, you can't drink!");
    } else {
        println!("Bruh, you can't drink!");
    }

    // Shorthand if else
    let is_office: bool = if age >= 21 { true } else { false };
    println!("Is office: {}", is_office);
}
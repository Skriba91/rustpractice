// Variables hold primitive data or references to data.
// Variables are immutable by default.
// Rust is a block-scoped language.

pub fn run() {
    let name = "Brad";
    
    let mut age = 31;
    println!("My name is {} and I am {}", name, age);

    age = 32;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables at once
    let (my_name, my_age) = ("Brad", 31);
    println!("{} is {}", my_name, my_age);
}
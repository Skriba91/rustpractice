pub fn run() {
    
    //Print to console
    println!("Hello, world from the print.rs file!");

    //Basic formatting
    println!("{} is from {}", "Brad", "Mass");

    //Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "Code");

    //Named arguments
    println!("{name} likes to {activity}", name = "Brad", activity = "Code");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait, print a touple
    println!("{:?}", (12, true, "hello"));

    //Basic math
    println!("10 + 10 = {}", 10 + 10);

}
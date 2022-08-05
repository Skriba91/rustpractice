use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    println!("Args: {:?}", args);

    println!("Command: {}", command);

    match command.as_str() {
        "hello" => println!("Hello world!"),
        "goodbye" => println!("Goodbye world!"),
        _ => println!("Invalid command"),
    }

}
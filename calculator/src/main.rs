use std::env::{args, Args};

fn main() {
    //Accessing the input arguments, Args is a struct that implements Iterator
    //To access specific elements with args.nth(), finction args has to be muatble
    let mut args: Args = args();

    //Printing all the arguments
    println!("{:?}", args);

    //arg is an iterator, every time nth is called it iterates the input arguments
    //to get it work properly, we need the set to the first element we need then
    //only call with 0
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    //Converting the string to a number, and define the type option 1
    //let first_number: f32 = first.parse().unwrap();
    //let second_number: f32 = second.parse().unwrap();

    //Converting the string to a number, and define the type option 2 best practice
    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    
    println!("{} {} {}", first, operator, second);

    let result = operate(operator, first_number, second_number);
    println!("{}", result);

    println!("{}", output(first_number, operator, second_number, result));
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    /*if operator == '+' {
        first_number + second_number //same as return first_number + second_number;
    } else if operator == '-' {
        first_number - second_number
    } else if operator == '*' {
        first_number * second_number
    } else if operator == '/' {
        first_number / second_number
    } else {
        0.0
    }*/

    //Better way to avoid if-else statements

    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalid operator used"),
    }
}

fn output(first_number: f32, operator: char, second_number: f32, resut: f32) -> String {
    //Format returns a string
    format!("{} {} {} = {}", first_number, operator, second_number, resut)
}
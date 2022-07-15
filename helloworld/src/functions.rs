//Functions - Used to store blocks of code for re-use

pub fn run() {
    greetings("Hello", "Dori");

    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    //Closure
    let num3: i32 = 10;
    let add_num = |num1: i32, num2: i32| num1 + num2;
    println!("Closure Sum: {}", add_num(3, 3));
}

fn greetings(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 //returns the value of the expression if not using semicolon
}
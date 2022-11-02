// Loops - Used to iterate over a collection of items, or until a condition is met.

pub fn run() {
    
    let mut count = 1;

    // Infinite loop
    loop {
        count += 1;
        println!("Number: {}", count);

        //Break condition
        if count == 20 {
            break;
        }
    }

    count = 1;
    println!("While loop");

    // While loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }

        count += 1;
    }

    println!("For range loop");

    // For range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
        
}
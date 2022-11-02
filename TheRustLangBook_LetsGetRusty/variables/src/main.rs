fn main() {
    
    //Mutability
    //In rust all variables are immutable by default
    //To make a variable mutable we use the keyword mut

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //Constants
    //Const cannot be mutable. Also type must be specified.
    //Constans cant hold a value which is calculated at runtime. For example
    //constant cannot hold the return value of a function.
    //Rust allows numeric literals '_' to make numbers more readable
    const SUBSCRIBER_COUNT: u32 = 100_0000;
    println!("Subscriber count: {}", SUBSCRIBER_COUNT);

    //Shadowing
    //We can shadow a variable by using the same variable’s name and repeating
    //the use of the let keyword
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}")
}

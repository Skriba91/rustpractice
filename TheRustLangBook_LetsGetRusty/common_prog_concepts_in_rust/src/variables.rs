pub fn variables() {

    /*Mutabulity */
    
    //Variable in rust are inmutable by default
    let mut x = 5; //Addig mut keyword to declare a variable muatable
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    /*Constants */
    //Constants are always immutable, mut keyword is not allowed
    //Constants can be declared in any scope, including the global scope
    //Constants must be annotated with the type of the value they will hold
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);


    /*Shadowing */
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is: {}", x);
    }
    println!("The value of x is: {}", x);

}
// Primitive str = Immutable fix-lenngth string somewhere in memory.
// String = Growable, heap-allocated data structure - Use when you need to
// modify or own string data

pub fn run() {
    let hello = "hello";
    println!("{}", hello);

    // Get length
    println!("Length: {}", hello.len());

    //Use String
    let mut hello2 = String::from("Hello");

    // Push char
    hello2.push(' ');

    // Push string
    hello2.push_str("World");

    // Capacity in bytes
    println!("Capacity: {}", hello2.capacity());

    // Check if empty
    println!("Is empty: {}", hello2.is_empty());

    // Contains

    println!("Contains 'World': {}", hello2.contains("World"));

    // Replace
    println!("Replace: {}", hello2.replace("World", "There"));
    
    println!("{}", hello2); 

    // Loop through string by whitespace
    for word in hello2.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing, this will fail in compile time
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

}
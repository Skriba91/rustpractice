use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    // Add element at the end of vector
    numbers.push(6);
    numbers.push(7);

    println!("{:?}", numbers);

    // Remove element from vector
    numbers.pop();

    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get Vector length
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice, slices from index 1 to 3. 1 included, 3 excluded.
    //Basicly [1, 3)
    let slice: &[i32] = &numbers[1..3];

    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate vector values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);

}
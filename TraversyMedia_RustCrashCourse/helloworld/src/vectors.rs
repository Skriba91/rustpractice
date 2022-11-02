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

    //Vector can be extended with another vector or array
    let mut vector_to_extend: Vec<i32> = vec![1, 2, 3];
    let word: u16 = 0xABCD;
    let word_in_bytes = word.to_be_bytes();
    println!("Word in bytes: {:?}", word_in_bytes);
    println!("Extended Vec: {:?}", vector_to_extend);
    vector_to_extend.extend([3, 4, 5]);
    println!("Extended Vec: {:?}", vector_to_extend);
    
    /*let mut a = vec![1, 2, 3];
    let b = vec![4, 5, 6];

    a.extend(&b);
    assert_eq!(a, [1, 2, 3, 4, 5, 6]);
    assert_eq!(b, [4, 5, 6]);
    println!("Extended Vec: {:?}", a);*/

}
// Vectors - resizable arrays.

use std::mem;

pub fn run() {
    // [Type, length of the array].
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Re-assign value.
    numbers[2] = 20;

    // Add onto vector.
    numbers.push(5);
    numbers.push(6);

    // Pop last value off of vector.
    numbers.pop();


    println!("{:?}", numbers);
    
    // Get single value.
    println!("Single value: {}", numbers[0]);

    // Get Vector length.
    println!("Vector Length: {}", numbers.len());

    // Vectors are stack allocated.
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice.
    let slice: &[i32] = &numbers[0..2]; 
    println!("Slice: {:?}", slice);

    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values.

    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);


 }
/*

Primitive types --
- Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
    (Number of bits they take in memory.)    
- Flats: f32, f64
Boolean (bool)
Characters, (char)
Tuples - Lists
Arrays - Arrays are fixed length (vectors are growable arrays.)

Rust is a statically typed language, but the compiler can usually infer what type we want to use base on the value and how we use it.



*/



pub fn run() {

    // Default is i32.
    let x = 1;

    // f64 by default.
    let y = 2.5;

    // Add explicit type.
    let z: i64 = 444444444444444;

    // Find max size.
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression.
    let is_greater = 10 > 5;

    // Char is characterized by single quotes.
    // Can only be 1 character.
    // Typically used for single unicode characters or emojis.
    let a1 = 'a';
    let face = '\u{1f600}';


    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));




}
// Primitive str = immutable fixed length string somewhere in memory.
// String = growable heap-allocated data structure - use when you need to modify or own string data.


pub fn run() {
    // Primitive string.
    let hello = "Hello";

    // String
    let mut _hello = String::from("Hello ");

    // Get length.
    println!("Length: {}", hello.len());

    // The three functions below are only possible with the String data type.

    // Push char. Used for pushing unicode characters to the string.
    _hello.push('W');

    // Push string.
    _hello.push_str("orld!");

    // Capacity in bytes.
    println!("Capacity: {}", _hello.capacity());

    // Check if empty.
    println!("Is empty: {}", _hello.is_empty());

    // Check if string contains substring. e.g. if hello ("hello world"), contains world.
    println!("Contains 'World' {}", _hello.contains("World"));

    // Replace.
    println!("Replace: {}", _hello.replace("World", "There"));

    // Loop through string by whitespace.
    for word in _hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity.
    let mut s = String:: with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing.
    // Assertion testing will log something to the console if it fails.
    assert_eq!(2, s.len());
    // assert_eq!(11, s.capacity()); <- will fail :)


    println!("{}", _hello);

}
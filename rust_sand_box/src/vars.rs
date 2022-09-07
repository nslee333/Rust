// Variables hold primitive data or reference to data.
// Variables are immutable by default.
// Rust is a block scoped language.

pub fn run() {
    let name = "Brad";
    let mut age = 37;

    age = 38;

    println!("My name is {} and I am {}", name, age);

    // Define constant. All uppercase for const declaration.
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables.
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}
// Functions - Used to store blocks of code for re-use.

pub fn run() {
    greeting("Hello", "Jane");

    // Bind function values to variables.
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure.
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 3));

}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

// In Rust, when you want to return a value you leave the semi-colon off.

fn add(n1: i32, n2: i32) -> i32 { // Arrow syntax to define what type to return from the function.
    n1 + n2 // This will return the value of n1 + n2.
}
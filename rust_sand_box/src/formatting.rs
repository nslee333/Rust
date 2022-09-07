mod print;

fn main() {
    // Print from another file.
    print::run();

    // Print to console.
    println!("Number:{}", 1);

    // Basic formatting.
    println!("{} is from {}", "Nathan", "MacMillan");

    // Positional arguments.

    println!("{0} is from {1} and {0} likes to {2}",
     "Brad", "Mass", "code"
    );

    // Named arguments.
    println!("{name} likes to play {activity}",
     name = "John", 
     activity = "hockey"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal {:o}", 10, 10, 10);

    // Placeholder for debug trait. Tuple.
    println!("{:?}", (12, true, "hello"));

    println!("10 + 10 = {}", 10 + 10);
}
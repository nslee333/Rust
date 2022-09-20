fn main() {
    println!("{}", fibonacci(9));
}

fn fibonacci(number: i32) -> i32 {
    if number <= 1 {
        return number;
    }
    return fibonacci(number - 1) + fibonacci(number - 2);

}

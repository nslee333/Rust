fn main() {
   
    fahrenheit_to_celsius(100.0);
    celsius_to_fahrenheit(100.0);

}

fn fahrenheit_to_celsius(fahrenheit: f32) {
    fahrenheit - 32 / 1.8
}

fn celsius_to_fahrenheit(celsius: f32) {
    celsius * 1.8 + 32
}
use std::io;
fn main() {
    let mut fahrenheit_input = String::new();
    println!("Convert temperatures between Fahrenheit and Celsius");
    println!("Please enter Fahrenheit: ");
    io::stdin().read_line(&mut fahrenheit_input).expect("Please input a number!");
    let fahrenheit_float: f32 = fahrenheit_input.trim().parse().expect("Its not a number!");
    let celsius: f32 = (fahrenheit_float - 32.0) * 0.5556;
    println!("{} Fahrenheit is {} Celsius", fahrenheit_float, celsius);
}

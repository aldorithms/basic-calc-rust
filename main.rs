use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().expect("Failed to read input"); // Read a line of input
    let mut values = input.split_whitespace(); // Split input by whitespace
    let c1: char = values.next().unwrap().parse().expect("Invalid char"); // Parse the first char
    if c1 != '=' 
    {
        panic!("c1 must be equal to '='");
    }
    let f1: f64 = values.next().unwrap().parse().expect("Invalid float"); // Parse the first float
    let c2: char = values.next().unwrap().parse().expect("Invalid char"); // Parse the second char
    let f2: f64 = values.next().unwrap().parse().expect("Invalid float"); // Parse the second float
    let result = match c2 
    {
        '+' => f1 + f2,
        '-' => f1 - f2,
        '*' => f1 * f2,
        '/' => f1 / f2,
        _ => panic!("c2 must be one of the following characters: +, -, *, /"),
    };
    println!("{} {} {} = {}", f1, c2, f2, result); // return result
}

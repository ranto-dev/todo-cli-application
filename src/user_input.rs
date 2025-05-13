use std::io;

// user input string 
pub fn input_string() -> String {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("error to insert string");
    return input;
}

// user input i32
pub fn input_i32() -> i32{
    let input: String = input_string();
    return input.trim().parse().expect("Error to parse string to i32")
}
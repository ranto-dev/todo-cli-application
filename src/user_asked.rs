use super::user_input::input_i32;

pub fn user_asked_to_continue_proram() -> i32 {
    println!("Whould you want to continue? If you want press 1 but else 0");
    println!("Your choice: ");
    let continue_choice: i32 = input_i32();
    println!("Your choice is {}", continue_choice);
    return continue_choice;
}
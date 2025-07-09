use std::io::{self};

fn main() {
    println!("Welcome, please enter your option (1-4)");
    println!("1 - New Note");
    println!("2 - Edit Note");
    println!("3 - Search Notes");
    println!("4 - Delete Notes");

    let mut user_option = String::new();

    io::stdin()
        .read_line(&mut user_option)
        .expect("Failed to read line");

    let trimmed_input = user_option.trim();

    let parsed_number: Option<i32> = match trimmed_input.parse::<i32>() {
        Ok(number) => Some(number),
        Err(_) => {
            println!("Error: '{trimmed_input}' is not a number.");
            None
        }
    };

    if let Some(input_num) = parsed_number {
        match input_num {
            1 => println!("You chose to add a note"),
            2 => println!("You chose to edit a note"),
            3 => println!("You chose to search for a note"),
            4 => println!("You chose to delete a note"),
            _ => {
                println!("Please choose a real option (1-4).");
            }
        }
    } else {
        println!("Please enter a number");
    }
}

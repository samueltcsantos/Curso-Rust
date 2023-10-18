use std::io;

fn main() {
    
    // Create a mutable variable
    let mut input = String::new();

    // Prompt the user to enter a number.
    println!("Please enter a number: ");

    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input as a number
    let seconds : Result<i32, _> = input.trim().parse();

    match seconds {

        Ok(n) => {
            // if parsing was successful, print the seconds
            println!("You entered : {} ", n);
        },
        Err(e) => {
            // If parsing failed, print an error message.
            println!("Error: {} ", e);
        }

    }

}

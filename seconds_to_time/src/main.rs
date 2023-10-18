use std::io;

mod lib;

fn main() {
    
    // Create a mutable variable
    let mut input = String::new();

    // Prompt the user to enter a number.
    println!("Please enter N seconds: ");

    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input as a number
    let seconds : Result<i32, _> = input.trim().parse();

    match seconds {

        Ok(n) => {
            // if parsing was successful, print the seconds
            let (days, hours, minutes, seconds) = lib::convert_seconds(n);
            println!("{} seconds is equal to: ", n);
            println!("{} d {}h {}m {}s", days, hours, minutes, seconds);
        },
        Err(e) => {
            // If parsing failed, print an error message.
            println!("Error: {} ", e);
        }

    }

}

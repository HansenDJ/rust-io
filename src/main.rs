use std::fs::{self, OpenOptions};
use std::io::{self, Write};

// Text file file path to read from and write to
static FILE_PATH: &str = "cars.txt";
static FAV_CAR_TXT: &str = "\nYour favorite car is a";

/// Reads the content of a text file and returns it as a String.
fn read_file(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}

/// Appends text from a string variable to the end of a text file.
fn append_to_file(file_path: &str, text: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true) // Open file in append mode
        .create(true) // Create the file if it doesn't exist
        .open(file_path)?;
    writeln!(file, "{}", text) // Write text with a newline
}

fn get_user_input() -> String {
    println!("Enter your favorite car year, make, and model: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.trim().to_string() // Return the input as a String
        }
        
        Err(error) => {
            println!("error: {error}");
            String::new() // Return an empty String in case of error
        }
    }
}

fn main() -> io::Result<()> {

    // Call the read_file function with original file
    match read_file(FILE_PATH) {
        Ok(content) => println!("Original file content:\n{}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    // Get user input from the terminal
    let user_input = get_user_input();

    println!("User entered: {}", user_input);

    // Combine favorite car text with the string the user inputs
    let combined_string = format!("{} {}", FAV_CAR_TXT, user_input);
    let combined_string_borrowed = combined_string.as_str();

    // Call the append_to_file function
    append_to_file(FILE_PATH, combined_string_borrowed)?;
    println!("Text appended to file.");

    // Call the read_file function with updated file
    match read_file(FILE_PATH) {
        Ok(content) => println!("Updated file content:\n{}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    Ok(())
}

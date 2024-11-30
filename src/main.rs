// use std::env;
// use std::fs;

// static FILE_PATH: String = "cars.txt";
// static FAVORITE_MAZDA_MODELS: String = "\nrx7, mx5, rx8, mazda6";

// fn read_file(file_path: String) {
//     // --snip--
//     println!("In file {file_path}");

//     let contents = fs::read_to_string(file_path)
//         .expect("Should have been able to read the file");

//     println!("With text:\n{contents}");

//     return contents;
// }

// fn write_file(file_path: String, write_data: String) {
//     fs::write(file_path, write_data).expect("Unable to write file");

//     // --snip--
//     // println!("In file {file_path}");

//     // let contents = fs::read_to_string(file_path)
//     //     .expect("Should have been able to read the file");

//     // println!("With text:\n{contents}");
// }

// fn main() {
//     let orig_text: String = read_file(FILE_PATH);
//     let combo_text = orig_text + &FAVORITE_MAZDA_MODELS;
//     write_file(FILE_PATH, combo_text);
// }

use std::fs::{self, OpenOptions};
use std::io::{self, Write};

static FILE_PATH: &str = "cars.txt";
static FAVORITE_MAZDA_MODELS: &str = "\nI also like the following Mazda car models: RX-7, MX-5, RX-8, Mazda6";

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

fn main() -> io::Result<()> {
    /*-------------TODO-------------*/
    // PRINT ORIGINAL TEXT FILE
    // GET USER INPUT ON WHAT TO APPEND
    // APPEND TO TEXT FILE
    // PRINT MODIFIED TEXT FILE

    // Call the append_to_file function
    append_to_file(FILE_PATH, FAVORITE_MAZDA_MODELS)?;
    println!("Text appended to file.");

    // Call the read_file function
    match read_file(FILE_PATH) {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    Ok(())
}

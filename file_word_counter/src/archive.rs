use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

fn read_file(file_path: &str) -> Result<String, Error> {
    // Open the file
    let file = File::open(file_path)?;

    // Create a BufReader to efficiently read the file line by line
    let reader = BufReader::new(file);

    // Read the content into a String
    let mut content = String::new();
    for line in reader.lines() {
        content.push_str(&line?);
        content.push('\n'); // If you want to preserve line breaks
    }

    Ok(content)
}

fn main() {
    let file_path = "data_files/data.txt";

    match read_file(file_path) {
        Ok(content) => {
            println!("File content:\n{}", content);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

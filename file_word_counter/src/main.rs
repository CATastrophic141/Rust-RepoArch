use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct LineReader<> {
    reader: BufReader<File>,
    current_line: String,
}

impl<'a> LineReader<> {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let current_line = String::new();

        Ok(Self {
            reader,
            current_line,
        })
    }

    pub fn get_next_line(&mut self) -> io::Result<Option<String>> {
        self.current_line.clear();
        match self.reader.read_line(&mut self.current_line)? {
            0 => Ok(None), // End of file
            _ => Ok(Some(self.current_line.clone())),
        }
    }

    pub fn get_current_line(&self) -> &str {
        &self.current_line
    }

    pub fn get_file_next_string(&mut self) -> io::Result<Option<String>> { //get_next_line() wrapper
        self.get_next_line()
    }

    pub fn get_file_next_char(&mut self) -> io::Result<Option<char>> {
        match self.get_next_line()? {
            Some(line) => Ok(line.chars().next()),
            None => Ok(None),
        }
    }

    pub fn get_file_next_uint(&mut self) -> io::Result<Option<u64>> { //uint 64
        match self.get_next_line()? {
            Some(line) => {
                let input = line.trim();
                let number_str = match input.find('.') {
                    Some(dot_index) => &input[..dot_index],
                    None => input,
                };

                if number_str.is_empty() {
                    return Ok(Some(0));
                }

                match number_str.parse::<u64>() {
                    Ok(num) => Ok(Some(num)),
                    Err(_) => Ok(None),
                }
            }
            None => Ok(None),
        }
    }

    pub fn get_file_next_int(&mut self) -> io::Result<Option<i64>> { //int 64
        match self.get_next_line()? {
            Some(line) => {
                let input = line.trim();
                let number_str = match input.find('.') {
                    Some(dot_index) => &input[..dot_index],
                    None => input,
                };

                if number_str.is_empty() {
                    return Ok(Some(0));
                }

                match number_str.parse::<i64>() {
                    Ok(num) => Ok(Some(num)),
                    Err(_) => Ok(None),
                }
            }
            None => Ok(None),
        }
    }

    pub fn get_file_next_unsigned_float(&mut self) -> io::Result<Option<f64>> { //float 64
        match self.get_next_line()? {
            Some(line) => {
                let input = line.trim();
                let number_str = match input.find('.') {
                    Some(dot_index) => &input[..dot_index],
                    None => input,
                };

                if number_str.is_empty() {
                    return Ok(Some(0.0));
                }

                match number_str.parse::<f64>() {
                    Ok(num) => Ok(Some(num)),
                    Err(_) => Ok(None),
                }
            }
            None => Ok(None),
        }
    }
}

fn main() {
    let file_path = "data_files/int_numbers.txt";
    let mut line_reader = match LineReader::new(file_path) {
        Ok(lr) => lr,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    while let Some(line) = line_reader.get_file_next_int().unwrap() {
        println!("geting line: {}", line);
        // User can handle the line here as needed
    }
}
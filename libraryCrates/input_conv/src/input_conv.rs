use std::fs::File;
use std::io::{self, BufRead, BufReader};
//use std::io;

//User IO handling

pub fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

pub fn read_charvec() -> Vec<char> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().chars().collect()
}

pub fn read_string_notrim() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.to_string()
}

pub fn read_u32() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim_start().to_string();

    if input.starts_with('-') {
        input = input[1..].to_string();
    }

    let number_str = match input.trim().find('.') {
        Some(dot_index) => &input[..dot_index],
        None => input.trim(),
    };

    if number_str.is_empty() {
        return 0;
    }

    number_str.parse().expect("Could not parse a valid unsigned 32-bit integer.")
}

pub fn read_i32() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number_str = match input.trim().find('.') {
        Some(dot_index) => &input[..dot_index],
        None => input.trim(),
    };

    if number_str.is_empty() || number_str == "-" {
        return 0;
    }

    number_str.parse().expect("Could not parse a valid 32-bit integer.")
}

pub fn read_u64() -> u64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim_start().to_string();

    if input.starts_with('-') {
        input = input[1..].to_string();
    }

    let number_str = match input.trim().find('.') {
        Some(dot_index) => &input[..dot_index],
        None => input.trim(),
    };

    if number_str.is_empty() {
        return 0;
    }

    number_str.parse().expect("Could not parse a valid unsigned 64-bit integer.")
}

pub fn read_i64() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number_str = match input.trim().find('.') {
        Some(dot_index) => &input[..dot_index],
        None => input.trim(),
    };

    if number_str.is_empty() || number_str == "-" {
        return 0;
    }

    number_str.parse().expect("Could not parse a valid 64-bit integer.")
}

pub fn read_f32() -> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Could not parse a valid floating-point number.")
}

pub fn read_f64() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Could not parse a valid floating-point number.")
}

pub fn read_u8() -> u8 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim_start().to_string();

    if input.starts_with('-') {
        input = input[1..].to_string();
    }

    let number_str = match input.trim().find('.') {
        Some(dot_index) => &input[..dot_index],
        None => input.trim(),
    };

    number_str.parse().expect("Could not parse a valid unsigned 8-bit integer.")
}

pub fn read_char() -> char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.chars().next().expect("Could not parse a valid character.")
}

/////// File Handling

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
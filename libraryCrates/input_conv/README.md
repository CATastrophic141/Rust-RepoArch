# Input Conversion Crate

## This crate contains a simple library used for reading user input and saving the input to a variable with of a given type.

## This crate intends to simplify the way input from a user is read, avoiding the reapeated handling and parsing of simple, single-type inputs.

### This is a introductory effort for rust crate publication, and may be unoptimized. *Use for official projects is unadvised.*

---

### Version History

Version 1.0.0: Added basic library for standard user inputs

Version 1.0.1: Updated README

Version 1.1.0: Added character vector and non-trimmed string return types. Added integer handling of improper decimals and '-' signs.

Version 1.1.1: Optimized sizing

Version 1.2.0: Added file line reading functions for iterator use

---

Available crate funcitons:
```rust
//Crate uses:
// std::fs::File;
// std::io::{self, BufRead, BufReader};

//Function names and return types:

/* User IO functions: */

pub fn read_string() -> String

pub fn read_charvec() -> Vec<char>

pub fn read_string_notrim() -> String

pub fn read_u32() -> u32

pub fn read_i32() -> i32

pub fn read_u64() -> u64

pub fn read_i64() -> i64

pub fn read_f32() -> f32

pub fn read_f64() -> f64

pub fn read_u8() -> u8

pub fn read_char() -> char

/**********/

/* File line reading info & functions: */
//The file reader should be used with an iterator

//A struct and implementation is used to track the current line position and data

pub struct LineReader<> {
    reader: BufReader<File>,
    current_line: String,
}

// The returned data exist as IO result options, and should be processed using <data>.unwrap() when the data needs to be used

impl<'a> LineReader<> {
    pub fn new(file_path: &str) -> io::Result<Self> 

    pub fn get_next_line(&mut self) -> io::Result<Option<String>> 

    pub fn get_current_line(&self) -> &str

    pub fn get_file_next_string(&mut self) -> io::Result<Option<String>> //string ; get_next_line() wrapper

    pub fn get_file_next_char(&mut self) -> io::Result<Option<char>> //char

    pub fn get_file_next_uint(&mut self) -> io::Result<Option<u64>> //uint 64

    pub fn get_file_next_int(&mut self) -> io::Result<Option<i64>> //int 64

    pub fn get_file_next_unsigned_float(&mut self) -> io::Result<Option<f64>> //float 64
}

/**********/
```

Example implementation of file iterator:
```rust
    let file_path = "./path/to/file.txt";
    let mut line_reader = match input_conv::LineReader::new(file_path) {
        Ok(lr) => lr,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    while let Some(line) = line_reader.get_file_next_string().unwrap() {
        println!("File line: {}", line);
    }

```

---

Potential updates:
Refactoring to split functionalities into separate modules
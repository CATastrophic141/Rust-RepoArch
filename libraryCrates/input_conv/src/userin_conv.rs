use std::io;

#[allow(non_snake_case)]

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
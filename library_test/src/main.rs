use input_conv;

//FUNCITONS IN input_conv CRATE:
///////////////////////
/*fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn read_charvec() -> Vec<char> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().chars().collect()
}

fn read_string_notrim() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.to_string();
}

fn read_u32() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Could not parse a valid unsigned integer.")
}

fn read_i32() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Could not parse a valid integer.")
}

fn read_u64() -> u64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Could not parse a valid unsigned 64-bit integer.")
}

fn read_i64() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Could not parse a valid 64-bit integer.")
}

fn read_f32() -> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Could not parse a valid floating-point number.")
}

fn read_f64() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Could not parse a valid floating-point number.")
}

fn read_u8() -> u8 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Could not parse a valid unsigned 8-bit integer.")
}

fn read_char() -> char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.chars().next().expect("Could not parse a valid character.")
}*/
///////////////////////////

fn main() {
    println!("Hello, world!");

    println!("Response (String):");
    let user_string = input_conv::read_string();
    println!("You entered: {}", user_string);

    println!("Untrimmed string:");
    let user_string = input_conv::read_string_notrim();
    println!("You entered: {}", user_string);

    println!("Float 64b num:");
    let user_num = input_conv::read_f64();
    println!("You entered: {}", user_num);

    println!("Float 32b num:");
    let user_num = input_conv::read_f32();
    println!("You entered: {}", user_num);

    println!("Unisnged 32b num:");
    let user_num = input_conv::read_u32();
    println!("You entered: {}", user_num);

    println!("Signed 32b num:");
    let user_num = input_conv::read_i32();
    println!("You entered: {}", user_num);
}

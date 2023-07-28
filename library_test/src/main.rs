use input_conv;

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

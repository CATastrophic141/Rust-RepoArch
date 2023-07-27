use std::io;
use std::io::*;

#[allow(non_snake_case)]

fn main() {
    println!("Welcome to the simple two-number, four-func calculator");

    println!("Enter the first number: ");
    stdout().flush().expect("Unable to flush stdout");

    let mut input_line1 = String::new();
    let mut input_line2 = String::new(); //Using the same input storage directly without clearing or using another causes erroneous behavior

    /* To read an integer */
    io::stdin() // the rough equivalent of `std::cin`
        .read_line(&mut input_line1) // actually read the line
        .expect("Failed to read line"); // which can fail, however
    //print!("Input: {}", input_line1);
    let firstNum: i64 = input_line1
        .trim() // ignore whitespace around input
        .parse() // convert to integers
        .expect("Input not an integer"); // which, again, can fail

    //println!("Number: {}", firstNum);

    println!("Enter the second number: ");
    stdout().flush().expect("Unable to flush stdout");

    io::stdin() // the rough equivalent of `std::cin`
        .read_line(&mut input_line2) // actually read the line
        .expect("Failed to read line"); // which can fail, however
    //print!("Input: {}", input_line2);
    let secondNum: i64 = input_line2
        .trim() // ignore whitespace around input
        .parse() // convert to integers
        .expect("Input not an integer"); // which, again, can fail

    //println!("Number: {}", secondNum);

    println!("Enter an operation to perform: [+] [-] [*] [/] (First char read only)");

    let mut input_char = String::new();
    io::stdin().read_line(&mut input_char).expect("Failed to read line");

    // We'll assume that the user only enters one character, so we'll take the first one.
    let character = input_char.chars().next().unwrap();

    match character {
        '+' => println!("{} + {} = {}", firstNum, secondNum, firstNum+secondNum),
        '-' => println!("{} - {} = {}", firstNum, secondNum, firstNum-secondNum),
        '*' => println!("{} * {} = {}", firstNum, secondNum, firstNum*secondNum as i64),
        '/' => println!("{} / {} = {}", firstNum, secondNum, firstNum/secondNum),
        // Add more cases as needed
        _ => println!("Error. Erroneous operation entered: [{}]", character),
    }
}

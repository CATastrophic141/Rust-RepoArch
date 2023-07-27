use std::io;

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u32;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let max_number: u32 = input.trim().parse().expect("Invalid input");

    println!("Prime numbers up to {}: ", max_number);
    for num in 2..=max_number {
        if is_prime(num) {
            println!("{}", num);
        }
    }
}
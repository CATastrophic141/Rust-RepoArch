use rand;
use input_conv;
fn main() {
    println!("Welcome to the number guessing game!\nI will pick a number between 1 and 100 (Inclusive).\nPlease guess which number I chose!");
    let mut number: u32 = rand::random::<u32>()%101;
    if number == 0 {
        number += 1;
    }
    let mut num_guesses = 1;
    let correct = false;
    while !correct {
        println!("Please guess the number:");
        let user_guess = input_conv::read_u32();
        if user_guess == number {
            break;
        }
        else if user_guess < number {
            println!("Your guess is too low");
        }
        else if user_guess > number {
            println!("Your guess is too high");
        }
        num_guesses += 1;
    }
    println!("You guessed it! You guessed the number after {} guesses", num_guesses);
}

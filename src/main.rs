use std::io;
use rand::Rng;

fn main() {
    println!("Guess is number");


    println!("input your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed");

    let integer_guess:_ = guess.trim().parse::<u32>().expect("enter an integer");
    let secret_number = rand::thread_rng().gen_range(1..=10);
    // println!("secret number {secret_number}");

    if integer_guess == secret_number{
        println!("you win")
    }else {
        println!("try again")
    }
}

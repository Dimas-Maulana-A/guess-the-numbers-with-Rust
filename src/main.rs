use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    loop{
        println!("input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed");

        let integer_guess: _ = guess.trim().parse::<u32>().expect("enter an integer");
        let secret_number = rand::thread_rng().gen_range(1..=5);
        // println!("secret number {secret_number}");

        match integer_guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too Big"),
            Ordering::Equal => {
                println!("You are Win");
                break;
            },
        }
    }

    // loop{
    //     if integer_guess == secret_number{
    //     println!("you win");
    //     break;
    // }else {
    //     println!("try again");
    // }
    // }
}

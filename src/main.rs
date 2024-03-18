use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng().gen_range(1, 101);
    let mut count: i32 = 0;

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number \n");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("{}", "Too small! \n".red());
                count += 1;
            }
            Ordering::Equal => {
                count += 1;
                println!("{}", "You won!".green());
                if count == 1 {
                    print!("{}", "You guessed it in ".yellow());
                    print!("{}", count.to_string().yellow());
                    println!("{}", " time".yellow());
                } else {
                    print!("{}", "You guessed it in ".yellow());
                    print!("{}", count.to_string().yellow());
                    println!("{}", " times".yellow());
                }
                break;
            }
            Ordering::Greater => {
                println!("{}", "Too big! \n".red());
                count += 1;
            }
        }
    }
}

use std::io;
use std::cmp::Ordering;
use colored::*;

fn main() {

    let secret_number = rand::random_range::<u16,_>(1..100);
    println!(":: Guessing Game ({}) ::", secret_number);

    loop {
        println!("Enter a number (1-100) :");
        let mut guess = String::new(); // mut if the value need to changed;
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("Your input is: {}", guess);

        let guess:u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Invalid input!".red());
                continue;
            },
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
        println!("The secret number is: {}", secret_number);
    }
}

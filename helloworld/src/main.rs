use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // let is defining variable
    // mut means its mutable
    // String::new() returns a new instance of a String
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // the read value into where the memory address for mutable guess is (so into variable guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Needs to be a number");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Guessed right!");
                break;
            }
        }
    }

    println!("The number was {}", secret_number);

    /*println!("You guessed: {}", guess);

    let x = 5;
    let y = 10;

    println!("x = {} and y + 2 = {}", x, y + 2);*/

    /* -------------------------------------- */
}

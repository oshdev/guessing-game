use std::cmp::Ordering;
use std::io;

use rand::Rng;

const LOW: u32 = 1;
const HIGH: u32 = 10;

fn main() {
    let secret_number = rand::thread_rng().gen_range(LOW, HIGH + 1);

    loop {
        println!("Guess the number in range {} - {} or q to quit.", LOW, HIGH);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read the line");

        if guess.trim() == "q" {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

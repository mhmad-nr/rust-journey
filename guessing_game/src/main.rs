use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("starting...");

    let mut guess = String::new();

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("this is {guess}");
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

    println!("you gessed {}", guess);
}

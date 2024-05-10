use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    let mut tries: i32 = 5;

    loop {
        println!("--------------------------------");
        println!("You have try {tries} times:");
        tries -= 1;

        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(_) => (),
            Err(_) => println!("Something went wrong"),
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
        if tries == 0 {
            println!("You Loss 🙂");

            break;
        }
    }
}

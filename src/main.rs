use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println! {"guess the number!"};
    println! {"Input guess from 1 to 101"};
    loop {
        let secret_number = rand::thread_rng().gen_range(1, 101);
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Couldn't read line");
        let f_guess = guess.trim_right().len();
        guess.truncate(f_guess);
        println!(
            "You guessed: {} and the secret number was {}",
            guess, secret_number
        );

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Right on the money!");
                break;
                }
        }
    }
}

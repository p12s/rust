use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!("🦀 this is rust game 🦀");

    print!("What is your name? ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Can't read the line");
    let name = name.trim();
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Hi, {}! Guess the number from 1 to 100.", name);

    loop {
        print!("Your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Can't read the line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Please enter a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("📈 Too small!"),
            std::cmp::Ordering::Greater => println!("📉 Too long!"),
            std::cmp::Ordering::Equal => {
                println!(
                    "🎉 Congratulations! You guessed the number {}!",
                    secret_number
                );
                break;
            }
        }
    }
}

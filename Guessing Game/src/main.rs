use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); // This generate a range between 1 and 100

    //looping through the game
    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Low! Try Again"),
            Ordering::Greater => println!("Too High! Try Again"),
            Ordering::Equal => {
                println!("Congratulations, You Win!!!");
                break;
            }
        }
    }
}

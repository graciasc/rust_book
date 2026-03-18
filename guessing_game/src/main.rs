use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("This is the start of a guess game \n\n");
    // random number
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        // check the user input
        let mut guess: String = String::new();
        println!("Please enter in a guess: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to  read line");

        // it's probably the parse
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        match guess.cmp(&random_number) {
            Ordering::Less => println!("The number you guess is low"),
            Ordering::Greater => println!("The number you guessed is to high"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }

    println!("The secret number is: {}", random_number);
}

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("This is the start of a guess game \n Please enter in a guess: ");
    // check the user input
    let mut guess: String = String::new();

    // random number
    let mut random_number: i32 = rand::thread_rng().gen_range(1..=100);
    // what does  &mut do in this case?
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to  read line");

    // it's probably the parse
    let guess: i32 = guess.trim().trim().parse().expect("Please enter a number!");
    println!("You guessed: {guess}");

    match guess.cmp(&random_number) {
        Ordering::Less => println!("The number you guess is low"),
        Ordering::Greater => println!("The number you guessed is to high"),
        Ordering::Equal => println!("You got it"),
    }
    println!("The secret number is: {}", random_number);
}

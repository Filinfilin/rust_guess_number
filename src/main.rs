use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range::<u32,_>(1..101);
    println!("Please input your guess.");
    let mut guess = String::new();


    io::stdin()
        .read_line(&mut guess)
        .expect("Filed to read line");
    
        let input: u32 = guess
        .trim()
        .parse()
        .expect("Wanted a number");


    match input.cmp(&secret_number) {
        Ordering::Less => println!("Too small! But number: {}", secret_number),
        Ordering::Greater => println!("Too big! But number: {}", secret_number),
        Ordering::Equal => println!("You win!"),
    };

    println!("You guessed:{}", guess);
}
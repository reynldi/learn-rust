use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number Game!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);
    
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            },
        };

        println!("Your guess is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small bruh!"),
            Ordering::Greater => println!("Too big bruh!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

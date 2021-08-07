use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    

    loop {
        println!("Input your guess.");

        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(nb) => nb,
            Err(_) => continue,
        };
    
        println!("Your guess {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Found!");
                break
            },
        }
    }


}

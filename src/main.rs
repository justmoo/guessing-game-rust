use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
     println!("welcome to the guessing game!");
     let secret_number = rand::thread_rng().gen_range(1..100);
     loop {
          println!("Type your guess");
          let mut guess = String::new();
          io::stdin().read_line(&mut guess).expect("Failed to read");

          println!("You guessed {}", guess);
          let guess: u32 = match guess.trim().parse(){
               Ok(num) => num,
               Err(_) => continue,
          };
          match guess.cmp(&secret_number) {
               Ordering::Less => println!("Too small"),
               Ordering::Greater => println!("too big"),
               Ordering::Equal => {
                    println!("You're right!");
                    break;
               },
          }
     }  
}

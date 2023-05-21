use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;
fn main(){


    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret Number is {}",secret_number);

    loop{
        let mut guess = String::new();
        println!("Enter a number to guess");

        io::stdin()
                .read_line(&mut guess)
                .expect("Error parsing Inout");

        let guess:i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
    
        println!("Number guessed is: {}",guess);
    
        match guess.cmp(&secret_number){
            Ordering::Greater => println!("{}","Bigger".red()),
            Ordering::Less => println!("{}","Smaller".red()),
            Ordering::Equal => {
                println!("{}","You Win".green());
                break;
            },

        }
    }
}
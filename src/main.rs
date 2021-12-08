use  std::io;   // For Input and output
use  rand::Rng; // For Randon Number Gentereation 
use  std::cmp::Ordering; // For Camparing two thing/number
use  colored::*; // For Coloring Library

fn main() {
    println!("Guess the Number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The Secret Number is  : {}", secret_number);        

    // Here is Loop to used take input Until you are not winner
     loop {
        
    println!("Please input your guess number");

    // Creating Variable rust is immutable but make it mutable need to add mut key word 
    // Take input from user

    let mut  guess = String::new();
    
    io:: stdin()
            .read_line(&mut guess)
            .expect("Failed to read line ");

    // Conver Guess input to int for guessing

    // let guess: u32 = guess .trim().parse().expect("Please type a number!");

    let guess: u32 = match guess .trim().parse(){
        Ok(num) =>num,
        Err(_)=> continue,

    };

    println!("You Guessed : {}", guess);      
    
    // Now we Campare 
    match guess.cmp(&secret_number){
        Ordering::Equal=> {
            println!("{}","You Are Winner!".green());
            break;
    },
        Ordering::Less=> println!("{}","Too Less".red()),
        Ordering::Greater=> println!("{}","Too Greater".red()),
    }
     }

}
 
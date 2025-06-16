use std::{cmp::Ordering, io};

use rand::Rng;
fn main() {
    println!("Number Guessing Game");
    
    let secret_number=rand::thread_rng().gen_range(1..=100);
   
    for i in 0..10{
    println!("Enter the nUmber");

    let mut guess= String::new();
    
   
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed To read line");
    let guess:u32=guess.trim().parse().expect("Please Enter a number");
    println!("You Guessed number ={}",guess);
    match guess.cmp(&secret_number) {
        Ordering::Greater =>println!("Too Big"),
        Ordering::Equal=> {println!("You winn");break;},
        Ordering::Less => println!("To Small"),
        
    }
}
}

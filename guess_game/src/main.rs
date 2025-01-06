use std::io;
fn main() {
    println!("**************GUESS THE NUMBER***************");
    println!("Enter your Guess: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failure to read input");
    println!("Input : {}",guess);
}


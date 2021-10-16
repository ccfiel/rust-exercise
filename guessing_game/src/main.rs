use std::io;
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..10);
    for number_of_quess in (0..3).rev() {
        let mut guess = String::new();
        println!("Please guess a number from 1 to 10: ");
        io::stdin().read_line(&mut guess).expect("Failed to readline");
        let guest: u32 = guess.trim().parse().expect("Please input a number");

        match guest.cmp(&secret_number) {
            std::cmp::Ordering::Equal => {
                print!("You are winner");
                break;
            },
            std::cmp::Ordering::Greater => println!("To big!"),
            std::cmp::Ordering::Less => print!("To small!")
        }    
        if number_of_quess != 0 {
            println!("You still have {} guesses", number_of_quess);
        } else {
            println!("game over! the secret number is {}", secret_number);
        }
    }
}

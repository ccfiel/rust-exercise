use std::io;
fn main() {
    let mut numb = String::new();
    println!("How many terms? ");
    io::stdin().read_line(&mut numb).expect("Input a number");
    let nterms: i32 = numb.trim().parse().expect("It is not a number");
    let mut n1 = 0;
    let mut n2 = 1;
    let mut count = 0;
    if nterms <= 0 {
        println!("Please enter a positive integer");
    } else if nterms == 1 {
        println!("Fibonacci sequence upto {} : {}", nterms, n1);
        
    } else {
        println!("Fibonacci sequence:");
        while count < nterms {
            println!("{}", n1);
            let ntn = n1 + n2;
            n1 = n2;
            n2 = ntn;
            count = count + 1;
        }
    }


}

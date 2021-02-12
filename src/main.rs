use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn user_guess() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number between 1 ad 100:");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("error while reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn computer_guess() {
    println!("Think a number between 1 and 100 and press enter.");
    io::stdin().read_line(&mut String::new()).expect("error");
    let mut high = 100;
    let mut low = 1;
    let mut mid: usize;
    loop {
        mid = (high + low) / 2;
        println!("Is your number equal {}?\n('yes', 'less' or 'more')", mid);
        let mut response = String::new();

        io::stdin()
            .read_line(&mut response)
            .expect("error while reading input");
        match response.trim() {
            "less" => high = mid,
            "more" => low = mid,
            "yes" => break,
            _ => println!("Please respond with 'yes', 'less' or 'more'"),
        }
    }
    println!("Your number is {}", mid);
}

fn main() {
    println!("Who should guess?\n1: You\n2: computer");
    loop {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("error while reading input");

        match choice.trim().parse() {
            Ok(1) => {
                user_guess();
                break;
            }
            Ok(2) => {
                computer_guess();
                break;
            }
            _ => continue,
        };
    }
}

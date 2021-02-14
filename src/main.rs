use rand::Rng;
use std::cmp::Ordering;
use std::io::{self,Write};

fn user_guess() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    print!("Guess a number between 1 ad 100.");

    loop {
        print!("Your guess: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("error while reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter an integer");
                continue;
            },
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
    print!("Think a number between 1 and 100 and press enter.");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).expect("error");
    let mut high = 100;
    let mut low = 1;
    let mut mid: usize;
    loop {
        mid = (high + low) / 2;
        print!("Is your number equal {}? [yes|less|more]: ", mid);
        io::stdout().flush().unwrap();
        let mut response = String::new();

        io::stdin()
            .read_line(&mut response)
            .expect("error while reading input");
        match response.trim() {
            "less" => high = mid,
            "more" => low = mid,
            "yes" => break,
            _ => println!("Please respond with yes, less or more"),
        }
    }
    println!("Your number is {}", mid);
}

fn main() { 
    loop {
        print!("Who should guess?\n1: You\n2: computer\nYour choice: ");
        let mut choice = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut choice)
            .expect("error while reading input");

        match choice.trim().parse() {
            Ok(1) => user_guess(),
            Ok(2) => computer_guess(),
            _ => continue,
        };

        choice = String::new();

        print!("Try again? [yes|no]: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut choice)
            .expect("error while reading input");

        match choice.trim() {
            "yes" => continue,
            _ => break,
        };
    };
}

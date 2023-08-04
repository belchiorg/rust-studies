use rand::Rng;

fn handle_value(correct_number: u8, guess: u8) -> bool {
    if correct_number < guess {
        println!("Secret number is lower!");
    }
    else if correct_number > guess {
        println!("Secret number is higher!");
    }
    else {
        return true
    }

    false
}

fn main() {
    let random_number: u8 = rand::thread_rng().gen_range(1..=100);
    //println!("Random Number ; {}", random_number);
    
    let mut guess = String::new();

    let mut guesses: u8 = 5;

    println!("Welcome to this game! You are given 5 chances to guess the random number!");

    
    while guesses > 0 {
        println!("Guesses left: {}", guesses);
        println!("What is your guess?");
        std::io::stdin().read_line(&mut guess).expect("Couldn't read line");
        let result = match guess.trim().parse::<u8>() {
            Ok(numeric) => handle_value(random_number, numeric),
            Err(_) => {
                println!("Invalid Number");
                guesses += 1;
                false
            },
        };
        
        if result {
            println!("Congrats, you won the game!");
            break;
        } 
        else {
            guesses -= 1;
        }

        guess = String::new();
    }

    if guesses <= 0 {
        println!("You lost and you are bad :P");
    }

}

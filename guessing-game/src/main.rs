use std::io::{Write, stdin, stdout};

fn main() {

    let num: u32 = rand::random_range(0..=100);

    println!("\n\x1b[4;32;1m----- Number Gussing Game -----\x1b[0m\n");
    
    println!("Guess the number. \nThe number is a integer between \x1b[34m0\x1b[0m to \x1b[34m100\x1b[0m. (Enter \"q\" to exit.)");

    let mut try_count = 0;

    loop {

        try_count += 1;
        
        print!("\nGuess! > ");

        if let Err(_) = stdout().flush() {
            panic!("Error occured.");
        }

        let mut buf = String::new();

        if let Err(_) = stdin().read_line(&mut buf) {
            panic!("Error occured.");
        }

        let input = buf.trim();

        if input.is_empty() {
            println!("Please enter any value.");
            continue;
        }

        if input.eq_ignore_ascii_case("q") {
            println!("Bye!");
            return;
        }

        let input = input.parse::<u32>();

        if let Err(_) = input {
            panic!("The given value is not a integer.");
        }

        let input = input.unwrap();

        if input < num {
            println!("The number is greater than {}.", input);
            continue;
        }

        if num < input {
            println!("The number is less than {}.", input);
            continue;
        }

        if num == input {
            println!("Correct!");
            break;
        }
    }

    println!("\n\x1b[34m--- Score --- \x1b[0m");

    println!("Point: {} \nTry: {} times", 100 - try_count, try_count);
}

use std::io;
use regex::Regex;
mod modules;

fn main() {
    let re = Regex::new(r"^[1-4]$").unwrap();

    loop {
        println!("=== Welcome to Student Registration ===");
        println!();
        println!("Choose an option to proceed.");
        println!();
        println!("1 - Register student");
        println!("2 - Show student data");
        println!("3 - Change student data");
        println!("4 - Exit");
        println!();

        let mut option: String = String::new();
    
        println!("Your choice: ");
    
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read input.");

        let option = option.trim();
    
        if re.is_match(option) {
            let option: u32 = option.parse().unwrap();
            choice_option(option);
        } else {
            println!("Invalid input. Please try again.")
        }
    } 
}

fn choice_option(option: u32) {
    match option {
        1 => println!("Register student"),
        2 => println!("Show student data"),
        3 => println!("Change student data"),
        4 => {
            println!("Shutting down...");
            std::process::exit(0);
        }

        _ => println!("Invalid input. Please try again."),
    }
}



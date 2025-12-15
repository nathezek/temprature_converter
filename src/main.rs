use std::io::{self, stdout, Write};

fn main() {
    println!(
        r#"
         ████████ ███████ ███    ███ ██████        ████████ ██    ██ ██ 
            ██    ██      ████  ████ ██   ██          ██    ██    ██ ██ 
            ██    █████   ██ ████ ██ ██████  █████    ██    ██    ██ ██ 
            ██    ██      ██  ██  ██ ██               ██    ██    ██ ██ 
            ██    ███████ ██      ██ ██               ██     ██████  ██ 
   "#
    );

    // Basic usage//
    // println!("\nCommands: ['1' to convert °F to °C ] | [ '2': to convert °C to °F ] | [ '3' to convert °C to K ] | [ '4' to convert °F to k ]");
    // Loop helps with re-running the compute for another value.
    'main_menu: loop {
        println!("\n------ Main Menu -------");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("q. Quit program");

        print!("Enter your choice (1, 2, or q): ");
        stdout().flush().expect("Failed to flush");

        // Created a mutable input variable to accept use input.
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        // Check if user enter 'q' before parsing the value.
        let choice = choice.trim();

        match choice {
            "q" | "quit" | "clear" => {
                break 'main_menu;
            }
            "1" => {
                println!("\nEntering Celsius to Fahrenheit mode. Type 'q' to return to main menu.");
                'c_to_f_loop: loop {
                    if let Some(celsius) = get_input("Enter a celsius number: ") {
                        let fahrienheit = celsius_to_fahrienheit(celsius);
                        println!("Result: {}°C = {:.2}°F", celsius, fahrienheit);
                    } else {
                        break 'c_to_f_loop;
                    }
                }
            }
            "2" => {
                println!("\nEntering Fahrenheit to Celsius mode. Type 'q' to return to main menu.");
                'f_to_c_loop: loop {
                    if let Some(fahrienheit) = get_input("Enter a celsius number: ") {
                        let celsius = fahrienheit_to_celsius(fahrienheit);
                        println!("Result: {}°F = {:.2}°C", fahrienheit, celsius);
                    } else {
                        break 'f_to_c_loop;
                    }
                }
            }
            _ => {
                println!("Invalid choice. Please select 1, 2, or q.");
            }
        }
    }
}

// ---------------------------------- COMPUTE FUNCTIONS --------------------------------- //
fn celsius_to_fahrienheit(celsius: f32) -> f32 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrienheit_to_celsius(fahrienheit: f32) -> f32 {
    (fahrienheit - 32.0) * 5.0 / 9.0
}

// ----------------------------------- HELPER FUNCTIONS -------------------------------- //

// User input parser function.
fn get_input(prompt: &str) -> Option<f32> {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read_line");
    let input = input.trim();

    if input == "q" || input == "quit" || input == "exit" || input == "clear" {
        return None;
    }

    match input.parse::<f32>() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Not valid input, please enter a valid number or 'q' to quit.");
            None
        }
    }
}

use std::io::{self, stdout, Write};

// ------------- Compute --------------------- //
fn compute(input: f32) -> f32 {
    (input - 32.0) * 5.0 / 9.0
}

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
    println!("\nCommands: ['1' to convert °F to °C ] | [ '2': to convert °C to °F ] | [ '3' to convert °C to K ] | [ '4' to convert °F to k ]");
    // Loop helps with re-running the compute for another value.
    loop {
        print!("\nEnter a number to convert: ");
        stdout().flush().expect("Failed to flush");

        // Created a mutable input variable to accept use input.
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Check if user enter 'q' before parsing the value.
        let input = input.trim();

        match input {
            "q" | "quit" | "clear" => {
                break;
            }
            _ => {
                // Now change to a valid type for compute and compute the value with function.
                let input: f32 = match input.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Not a command");
                        continue;
                    }
                };

                let result = compute(input);
                println!("Result is : {}°C", result)
            }
        }
    }
}

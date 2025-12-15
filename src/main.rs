use std::io::{self, stdout, Write};

// ------------- Compute --------------------- //
fn compute(input: i32) -> i32 {
    (input - 32) * 5 / 9
}

fn main() {
    println!(
        r#"
                                                                                                                     
                                                                                                                     
████████ ███████ ███    ███ ██████       ██████  ██████  ███    ██ ██    ██ ███████ ██████  ████████ ███████ ██████  
   ██    ██      ████  ████ ██   ██     ██      ██    ██ ████   ██ ██    ██ ██      ██   ██    ██    ██      ██   ██ 
   ██    █████   ██ ████ ██ ██████      ██      ██    ██ ██ ██  ██ ██    ██ █████   ██████     ██    █████   ██████  
   ██    ██      ██  ██  ██ ██          ██      ██    ██ ██  ██ ██  ██  ██  ██      ██   ██    ██    ██      ██   ██ 
   ██    ███████ ██      ██ ██           ██████  ██████  ██   ████   ████   ███████ ██   ██    ██    ███████ ██   ██ 
                                                                                                                     
                                                                                                                     
    "#
    );

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
                let input: i32 = input.parse().expect("Mismatched type here.");
                let result = compute(input);
                println!("Result is : {}°C", result)
            }
        }
    }
}

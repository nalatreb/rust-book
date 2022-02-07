use std::io;

fn main() {
    loop {
        let mut user_input = String::new();
        println!("Please enter the temperature in celsius.");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error while reading from input.");

        if user_input.trim() == "quit" {
            break;
        }

        let result = match user_input.trim().parse::<f64>() {
            Ok(num) => (num * 9.0 / 5.0) + 32.0,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("The temperature is {} Fahrenheit.", result);
    }
}

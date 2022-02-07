fn main() {
    loop {
        println!("Please enter a number of fibonacci you want.");
        let mut user_input = String::new();

        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Error while reading user input.");
        let user_input: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a valid number.");
                continue;
            }
        };

        println!(
            "The {} fibonacci number is: {}",
            user_input,
            calculate_fibonacci(user_input)
        );
    }
}

fn calculate_fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2);
}

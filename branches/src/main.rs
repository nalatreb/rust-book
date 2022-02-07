fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number_2 = 2;
    if number_2 != 0 {
        println!("number_2 is not null");
    }

    let number_3 = 6;

    if number_3 % 4 == 0 {
        println!("numer_3 is divisible by 4");
    } else if number_3 % 3 == 0 {
        println!("numer_3 is divisible by 3");
    } else if number_3 % 2 == 0 {
        println!("numer_3 is divisible by 2");
    } else {
        println!("number_3 is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number_4 = if condition { 5 } else { 6 };
    println!("The value of number_4 is: {}", number_4);
}

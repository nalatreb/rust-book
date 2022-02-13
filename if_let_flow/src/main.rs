fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!("Count: {}", count);

    let mut count = 0;
    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("Count: {}", count);
}

enum Coin {
    Quarter(UsState),
    Penny,
}

#[derive(Debug)]
enum UsState {
    Alabama,
}

use std::collections::HashMap;

fn main() {
    let mut numbers = vec![
        2, 22, 1, 3, 51, 32, 1, 22, 3, 4, 5, 12, 32, 51, 23, 51, 15, 1, 1, 1, 1,
    ];
    numbers.sort();
    let median = median(&numbers);
    println!("The median is: {}", median);

    let mode = mode(numbers);
    println!("The mode is: {}", mode);
}

fn median(numbers: &Vec<i32>) -> f64 {
    let index = numbers.len() / 2;
    if numbers.len() % 2 == 1 {
        return numbers[index] as f64;
    }

    return (numbers[index] + numbers[index + 1]) as f64 / 2.0;
}

fn mode(numbers: Vec<i32>) -> i32 {
    let mut counter = HashMap::new();
    for number in numbers {
        let count = counter.entry(number).or_insert(0);
        *count += 1;
    }

    counter
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")

    // let mut max_index = 0;
    // let mut max = 0;
    // for (index, value) in counter {
    //     if value >= max {
    //         max_index = index;
    //         max = value;
    //     }
    // }

    // max_index
}

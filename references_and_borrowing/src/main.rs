fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}', is {}.", s1, len);

    let mut s2 = String::from("hello");

    change(&mut s2);
    println!("{}", s2);

    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;

    let mut s = String::from("hello");
    // no problem
    let r1 = &s;
    // no problem
    let r2 = &s;
    // BIG PROBLEM
    // let r3 = &mut s;

    // println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");

    // no problem
    let r1 = &s;
    // no problem
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    // no problem
    let r3 = &mut s;
    println!("{}", r3);

    let reference_to_nothing = dangle();
}

// dangle returns a reference to a String
// fn dangle() -> &String {
//     // s is a new String
//     let s = String::from("hello");

//     // we return a reference to the String, s
//     &s
// }
// Here, s goes out of scope, and is dropped. Its memory goes away. Danger!

fn dangle() -> String {
    let s = String::from("hello");

    s
}

// s is a reference to a String
fn calculate_length(s: &String) -> usize {
    s.len()
}
// Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

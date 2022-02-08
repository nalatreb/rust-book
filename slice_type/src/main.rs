fn main() {
    let mut s = String::from("hello world");

    // word will get the value 5
    let word = first_word(&s);

    // this empties the String, making it equal to ""
    s.clear();

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    println!("{}", word);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    let slice = &s[0..len];
    let slice = &s[..];

    let sliced_word = first_word(&s);
    println!("sliced_word: {}", sliced_word);
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// fn second_word(s: &String) -> (usize, usize) {

// }

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

    let mut s = String::from("hello");

    let word = first_word_slice(&s);

    // s.clear();

    println!("the first word is: {}", word);

    let s = "Hello, world!";
    let word = first_word_string(&s);

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_string(&my_string[0..6]);
    let word = first_word_string(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_string(&my_string_literal[0..6]);
    let word = first_word_string(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_string(my_string_literal);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word_string(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
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

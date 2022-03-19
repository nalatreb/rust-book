fn main() {
    let mut text = String::from("a string that must be convert");
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    let mut new_text = String::from("");
    for word in text.split_whitespace() {
        let first_char = word.chars().next().unwrap();
        if vowels.contains(&first_char) {
            new_text.push_str(&format!("{}-hay", word));
        } else {
            new_text.push_str(&format!("{}-{}ay", &word[1..word.len()], first_char));
        }
        new_text.push(' ');
    }

    println!("{}", new_text);
}

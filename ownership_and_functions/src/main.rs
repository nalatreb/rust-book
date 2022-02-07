fn main() {
    // s comes into scope
    let s = String::from("hello");

    // s's value moves into the function...
    takes_ownershop(s);
    // ... and so is no longer valid here

    // x comes into scope
    let x = 5;

    // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
    makes_copy(x);
}
// Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

// some_string comes into scope
fn takes_ownershop(some_string: String) {
    println!("{}", some_string);
}
// Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

// some_integer comes into scope
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
// Here, some_integer goes out of scope. Nothing special happens.

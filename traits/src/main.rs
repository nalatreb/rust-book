use std::fmt::{Debug, Display};

use traits::{NewsArticle, Summary, Tweet};

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

pub fn notify3(item1: &impl Summary, item2: &impl Summary) {}

pub fn notify4<T: Summary>(item1: &T, item2: &T) {}

pub fn notify5(item: &(impl Summary + Display)) {}

pub fn notify6<T: Summary + Display>(item: &T) {}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    3
}

fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    3
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_book"),
        content: String::from("of course, as you probable already know, people"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x)
        } else {
            println!("The largest member is y = {}", self.y)
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_book"),
        content: String::from("of course, as you probable already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());
}

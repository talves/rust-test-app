#![allow(unused)]
mod front_of_house; // tells rust to load from front_of_house.rs

pub mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

use crate::back_of_house::Appetizer;
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
    // Absolute path
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    // this is a private function, not accessible
    // front_of_house::hosting::seat_at_table();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

// Traits

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("author is ({})", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// impl TRAIT for TYPE
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("author is ({})", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// public function
pub fn adder(a: i32, b: i32) -> i32 {
    (a + b)
}

// private function
fn prints_and_returns_value(a: i32) -> i32 {
    println!("I got the value {}", a);
    a
}

// Tests (unit tests)
#[cfg(test)]
mod tests {
    use super::*; // allows tests to use private functions (brings into scope)

    // if you run with parallel (multiple threads), cargo test -- --show-output
    // to run syncronous cargo test -- --test-threads=1
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_value(4);
        assert_eq!(4, value);
    }

    #[test]
    #[ignore] // this fails with testing (cargo test -- --ignored)
    fn this_test_will_fail() {
        let value = prints_and_returns_value(8);
        assert_eq!(6, value); // should be 8
    }
}

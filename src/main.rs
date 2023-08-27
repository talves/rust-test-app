#![allow(unused)]
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, ErrorKind, Read},
};

use rust_test_app::{NewsArticle, Summary, Tweet};

fn main() {
    scope_example();
    ownership_example();
    borrow_call();
    mutable_references();
    slice_type();
    array_slice();
    struct_example();
    tuple_example();
    emum_values();
    enum_example();
    enum_option();
    enum_control_flow();
    pattern_matching();
    calling_a_module_function();
    common_collections();
    using_hash_map();
    // Error handling
    crash_n_burn();
    propagating_error();
    // generics
    generics();
    // traits
    traits();
    // lifetimes
    lifetimes();
    // traits
    dynamic_traits();
}

fn scope_example() {
    let s = "test";
    {
        // s is now setup within this scope
        let mut s = s; // s is valid from this point forward

        // do stuff with s
        println!("Inner s is a copy: {}", s);
        s = "done.";
        println!("Inner s changed to: {}", s);
    }
    println!("The value is: {}", s);
}

fn ownership_example() {
    let s1 = String::from("hello");
    // let s2 = s1; // moved into s2, so s1 is invalidated, using will panic!
    let s2 = s1.clone(); // copied into s2, so s1 is still valid!
                         // s1.push_str(", world!");

    println!("s1 value: {}", s1); // panics here if invalidated
    println!("s2 value: {}", s2);

    let n1 = 5;
    let n2 = n1; // makes a copy of the value into n2 (not move)
    println!("n1 value: {}", n1);
    println!("n2 value: {}", n2);

    let some_string = gives_ownership("yo, do it!".to_string());
    println!("some_string value: {}", some_string);
}

fn gives_ownership(a_string: String) -> String {
    // a_string comes into scope

    // let some_string = String::from("yours"); // some_string comes into scope

    a_string // a_string is returned and
             // moves out to the calling
             // function
}

fn borrow_call() {
    let s1 = String::from("hello");

    let len = borrow_and_len(&s1);

    println!("The length of '{}' is {} (borrowing).", s1, len);

    let s2 = s1.clone(); // expensive copy of a string. (borrowing above is more performant)
    let len = length_of_string(s1); // s1 gets invalidated here.
    println!("The length of '{}' is {} (borrowing).", s2, len);
}

fn borrow_and_len(s: &String) -> usize {
    s.len()
}
fn length_of_string(s: String) -> usize {
    s.len()
}

fn mutable_references() {
    let mut s = String::from("hello");

    change_ref(&mut s);
    println!("value after change_ref: {}", s);
    change_ref(&mut s);
    println!("value after change_ref (2nd call): {}", s);
}
fn change_ref(some_string: &mut String) {
    some_string.push_str(", world");
}

fn slice_type() {
    let sentence = "Here is a sentence".to_string();
    let first = first_word_len(&sentence);
    println!("length of first word in '{}': {}", sentence, first);

    let first_word = &sentence[0..first]; // (or &sentence[..first]) a string slice. Woot!
    println!("first word is: {}", first_word);
}
fn first_word_len(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn array_slice() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    // let slice = &a[1..];

    println!("slice: {:?}", slice);

    assert_eq!(slice, &[2, 3]);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn struct_example() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("tony@example.com"),
        ..user1
    };

    println!("user1 email: {}", user1.email);
    println!("user2 email: {}", user2.email);
}

fn tuple_example() {
    let rect1 = (50, 90);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );
    let rect2 = Rectangle {
        width: 50,
        height: 30,
    };
    println!(
        "The area of the rectangle {:?} is {} square pixels.",
        rect2,
        area_struct(&rect2)
    );
    let rect3 = Rectangle {
        width: 60,
        height: 20,
    };
    println!(
        "The area of the rectangle {:?} is {} square pixels.",
        rect3,
        area_fn_struct(&rect3)
    );
}
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
fn area_fn_struct(rectangle: &Rectangle) -> u32 {
    rectangle.area()
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn emum_values() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("The IP address: {:?}", home);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("The message: {:?}", self);
    }
}

fn enum_example() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

#[derive(Debug)]
enum AppOption<T> {
    None,
    Some(T),
}

fn enum_option() {
    let some_number = AppOption::Some(5);
    let some_string = AppOption::Some("a string");

    let absent_number: AppOption<i32> = AppOption::None;
    println!("The none: {:?}", absent_number);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn enum_control_flow() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    California,
    Oregon,
    Washington,
    // --snip--
}

fn pattern_matching() {
    let mut coin = Coin::Dime;
    let coin_value = value_in_cents(coin);
    println!("The coin is worth {:?} cents!", coin_value);
}

fn calling_a_module_function() {
    rust_test_app::eat_at_restaurant();
}

fn common_collections() {
    // A vector allows you to store a variable number of values next to each other.
    let mut v: Vec<i32> = Vec::new();
    println!("Vector (v): {:?}", v);
    v = vec![1, 2, 3]; // vec is a macro used to store values
    println!("Vector (v): {:?}", v);
    v.push(12);
    v.push(24);
    println!("Vector (v): {:?}", v);
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(3) {
        Some(third) => println!("The fourth element is {}", third),
        None => println!("There is no third element."),
    }
    let first = &v[0];
    println!("The first element is: {}", first);
    v.push(48);
    println!("Vector (v): {:?}", v);
    for i in &v {
        println!("{}", i);
    }
    for i in &mut v {
        *i += 100;
        println!("{}", i);
    }
    // A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
    let data = "initial contents";
    let s = data.to_string();
    println!("String (s): {}", s);

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("String (s): {}", s);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("String (s): {}", s);

    // A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.
}

fn using_hash_map() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("HashMap (map): {:?}", map);
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("HashMap (scores): {:?}", scores);
}

fn crash_n_burn() {
    // panic!("Crash and burn!");
    // let v = vec![1, 2, 3];

    // v[99];
    read_missing_file();
    check_error_kind();
    let f = dynamic_error();
    println!("file (hello_ok.txt): {:?}", f);
}

fn read_missing_file() {
    // f will have a type enum Result(File, Error); Ok or Err
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    let f = File::open("hello_ok.txt");
    println!("file (hello_ok.txt): {:?}", f);
    let f = match f {
        Ok(file) => file,                                              // return file
        Err(error) => panic!("Problem opening the file: {:?}", error), // panic here, quit
    };
    // caught an error: file (hello.txt): Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
}

fn check_error_kind() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // creates the file if it doesn't exist
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    // The equivalent (better) code of the above
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Unwrapping the Ok
    let f = File::open("hello.txt").unwrap();
}

fn propagating_error() {
    println!("Calling propagating error");
    let f = read_username_from_file();
    println!("file (hello_ok.txt): {:?}", f);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

    // Equivalent shorter code for above using ? (returns an error if one)
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // OR even shorter than that
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)
}
fn dynamic_error() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    fn refx(&self) -> &f32 {
        println!("reference of x: {}", &self.x);
        &self.x
    }
}
fn generics() {
    let point: Point<i32> = Point { x: 5, y: 3 };
    println!("point: {:?} x:{} y:{}", point, point.x(), point.y);
    let point: Point<f32> = Point { x: 10.0, y: 10.0 };
    println!(
        "point: {:?} x:{} y:{} distance:{}",
        point,
        point.refx(),
        point.y,
        point.distance_from_origin()
    );
}

fn traits() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
    let tweet = returns_summarizable_tweet();
    println!("a new tweet: {}", tweet.summarize());
}

pub fn notify<T: Summary>(item: &T) {
    // this function has a trait bound to T (Summary trait)
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable_tweet() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// Does not compile right now
fn returns_summarizable_switch(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        })
    } else {
        Box::new(returns_summarizable_tweet())
    }
}

fn dynamic_traits() {
    let summary = returns_summarizable_switch(true);
    println!(
        "summary: {} {}",
        summary.summarize_author(),
        summary.summarize()
    );
    let summary = returns_summarizable_switch(false);
    println!(
        "summary: {} {}",
        summary.summarize_author(),
        summary.summarize()
    );
}

// Lifetimes

fn lifetimes() {
    println!("which: {}", longest("long", "longer"));
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

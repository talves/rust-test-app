use std::sync::mpsc::Receiver;

fn main() {
    scope_example();
    ownership_example();
    borrow_call();
    mutable_references();
    slice_type();
    array_slice();
    struct_example();
    tuple_example();
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
}
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

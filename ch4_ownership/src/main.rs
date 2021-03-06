fn main() {
    println!("Understanding Ownership");

    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    // Mutable references
    // You can only have one mutable reference to a particular piece of data
    // in a particular scope
    // Prevents data races

    // let r1 = &mut s1;
    // {
    //     let r2 = &mut s1;
    // }

    // Stack-only data : Copy
    let x = 5;
    let y = x;
    println!("x: {} & y: {}", x, y); // x is valid here since x is an i32 and it's value gets
                                     // copied into y rather than moved there. i32 implements the Copy trait

    // Heap allocated data: Clone
    let s_test = String::from("Hell");
    let s_test2 = s_test.clone(); // deep copies the heap data of the String, not just the stack data
    println!("stest = {}, stest2 = {}", s_test, s_test2);

    let s = String::from("ownership test"); // s comes into scope
    takes_ownership(s); // s's value moves into the function and is no longer valid here

    // A reference's scope starts where it's introduced and continues till the last time it's used
    let r1 = &s1;
    let r2 = &s1;
    println!("{}, {}", r1, r2);
    // r1 and r2 are no longer used after this so they aren't in scope

    let r3 = &mut s1;
    println!("{}", r3);

    // println!("{}, {}", r1, r2);
    // change(&mut s1);

    // let reference_to_nothing = dangle();

    let s = String::from("Hello");
    let s = s + &String::from(" World!");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    println!("The First word in '{}' is: '{}'", s, first_word(&s));
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope, 'drop' is called,
  // and the backing memory is freed

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Solution is to return the String directly rather than a ref to it
fn dangle() -> String {
    // returns a reference to a String
    let s = String::from("hello"); // s is a new String
    s
    // &s // return a ref to the String s
} // s goes out of scope and is dropped. It's memory goes away

fn change(some_string: &mut String) {
    // Compiler error: Cannot modify a borrowed value that is immutable
    some_string.push('o');
}

// Borrowing - Having references as function parameters
// s a reference to the string rather than taking ownership of it
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but since it does not have ownership of what it
  // refers to, nothing happens

use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; // Defines an associated type for the Deref trait to use

    fn deref(&self) -> &T {
        &self.0
    }
}

use crate::List::{Cons, Nil};

fn main() {
    println!("Chapter 15 - Smart Pointers");

    // Boxes store their data on the heap rather than the stack.
    // what remains on the stack is a pointer to the heap data
    let b = Box::new(5);
    println!("b = {}", b);

    // Recursive data type
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // Following the pointer to the value with the Dereference operator
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // deref y to access the int value y is pointing to

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // cannot be dereferenced since mybox doesn't implement
                       // the Deref trait
                       // The dereferencing works once we implement the trait
}

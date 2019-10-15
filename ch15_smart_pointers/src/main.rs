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

// Implementing the Deref trait
impl<T> Deref for MyBox<T> {
    type Target = T; // Defines an associated type for the Deref trait to use

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
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

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // deref coercion in action
    // Deref coercion converts a reference to a type that implements Deref
    // into a reference to a type that Deref can convert the original type into
    // Without deref coercion, hello(&m) would be written as -
    // hello(&(*m)[..])
    // When the Deref trait is defined for the types involved, Rust will analyze
    // the types and use Deref::deref as many times as necessary to get a reference
    // to match the parameter's type. The number of times is resolved at compile
    // time, so there's no runtime penalty for using this.

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let e = CustomSmartPointer {
        data: String::from("My Stuff also"),
    };
    println!("CustomSmartPointers created");
    drop(c);
    println!("CustomSmartPointer c dropped before end of main");
    {
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
    } // <- d goes out of scope here so the drop function is called
      // variables are dropped in reverse order of creation (LIFO)
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}
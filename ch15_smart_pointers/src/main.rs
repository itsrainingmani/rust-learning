#![allow(dead_code)]
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

// List that can have multiple owners using Rc
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

#[derive(Debug)]
enum RefList {
    Cons(Rc<RefCell<i32>>, Rc<RefList>),
    Nil,
}

#[derive(Debug)]
enum RefCycleList {
    Cons(i32, RefCell<Rc<RefCycleList>>), // This gives the ability to modify which List a Cons is pointing to
    Nil,
}

impl RefCycleList {
    fn tail(&self) -> Option<&RefCell<Rc<RefCycleList>>> {
        match self {
            RefCycleList::Cons(_, item) => Some(item),
            RefCycleList::Nil => None,
        }
    }
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

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

use crate::List::{Cons, Nil};
use crate::RcList::{Cons as RcCons, Nil as RcNil};
use crate::RefCycleList::{Cons as CycleCons, Nil as CycleNil};
use crate::RefList::{Cons as RefCons, Nil as RefNil};

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

    // Reference Counted Smart Pointer
    // Rc<T> enables multiple owners of the same data
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcCons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }
    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));

    // Having multiple owners of Mutable data by combining Rc<T> & RefCell<T>
    // Recall: Rc<T> lets you have multiple owners of some data but only gives
    // you immutable access to that data.
    // Having an Rc<T> that holds a RefCell<T>, you can get a value that has
    // multiple owners and mutate the value
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RefCons(Rc::clone(&value), Rc::new(RefNil)));

    let b = RefCons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = RefCons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // Creating a Reference Cycle
    let a = Rc::new(CycleCons(5, RefCell::new(Rc::new(CycleNil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(CycleCons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    // So far, we've demonstrated that calling Rc::clone increases the
    // strong_count of an Rc<T> instance and an Rc<T> instance is only
    // cleaned up if it's strong_count is 0

    // Preventing Ref Cycles by turning an Rc<T> to a Weak<T>
    // Strong references are how you can share ownership of an Rc<T> instance
    // Weak refs do not express an ownership relationship
    // let leaf = Rc::new(Node {
    //     value: 3,
    //     children: RefCell::new(vec![]),
    //     parent: RefCell::new(Weak::new()),
    // });

    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // leaf parent = None

    // let branch = Rc::new(Node {
    //     value: 5,
    //     children: RefCell::new(vec![Rc::clone(&leaf)]),
    //     parent: RefCell::new(Weak::new()),
    // });

    // *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // modify leaf to give it a weak ref to it's parent

    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!(
        "Leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "Branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "Leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "Leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    // Summary
    // * Box<T> has a known size and points to data allocated on the heap
    // * Rc<T> keeps track of the number of references to data on the Heap
    // so that data can have multiple owners
    // * RefCell<T> with its interior mutability gives us a type that we
    // can use when we need an immutable value but need to change an
    // inner value of that type. Also enforces borrowing rules at runtime
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

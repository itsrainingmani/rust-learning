use mylib::Summary;
use std::cmp::PartialOrd;
use std::fmt::Display;

// fn largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// Function is a generic over some type T.
// It has one parameter names list, which is a slice of values of type T
// It will return a value of the same type T

// Here the function's parameter type T is trait-bound to only accept types
// that implement the PartialOrd Trait allowing for > comparisons
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = &item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// the distance_from_origin function will only exist on Points with f32 type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointBoth<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointBoth<T, U> {
    fn mixup<V, W>(self, other: PointBoth<V, W>) -> PointBoth<T, W> {
        PointBoth {
            x: self.x,
            y: other.y,
        }
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

// Using Trait Bounds to Conditionally Implement Methods
// Pair<T> only implements the cmp_display method if it's inner type T
// implements the PartialOrd trait that enables comparison and the Display trait
// that enables printing
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The larger member is x = {}", self.x);
        } else {
            println!("The larger member is y = {}", self.y);
        }
    }
}

fn main() {
    println!("Chapter 10 - Generic Types, Traits, and Lifetime");

    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&number_list));

    let mut char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 123.3 };

    println!("Distance from origin: {}", float.distance_from_origin());

    let char_and_float = PointBoth { x: 'a', y: 4.6 };
    println!("integer.x = {}", integer.x());

    let only_int = PointBoth { x: 1, y: 3 };
    let p3 = char_and_float.mixup(only_int);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = mylib::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let tumble = mylib::TumblrPost {
        username: String::from("bsbarbie"),
        content: String::from("I am 14 and this is deep"),
        notes: 114_121,
        reply: false,
    };

    // println!("1 new post: {}", tumble.summarize());
    // mylib::notify(tumble);
    // mylib::notify(tweet);

    mylib::notify_both(tweet, tumble);

    let _summarized = mylib::returns_summarizable();
}

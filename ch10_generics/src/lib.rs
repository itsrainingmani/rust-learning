pub trait Summary {
    // fn summarize_author(&self) -> String;
    // fn summarize(&self) -> String {
    //     format!("(Read more from {}...)", self.summarize_author()) // Default behavior
    // }
    // fn summarize(&self) -> String; // The Summary trait defines a summarize method
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// This function expects it's item parameter to be some type that implements the Summary trait
// pub fn notify(item: impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// item must be of type that implements both Summary and Display traits
// pub fn notify(item: Summary + Display) {}

// Trait bound notation for above function
// pub fn notify<T: Summary + Display>(item: T) {}

// Trait bound notation
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound - contrains both parameters to be same type that implements Summary trait
// pub fn notify_both<T: Summary>(item1: T, item2: T) {}

pub fn notify_both(item1: impl Summary, item2: impl Summary) {
    println!(
        "Breaking news! \n\t{}\n\t{}",
        item1.summarize(),
        item2.summarize()
    );
}

// Specify trait bounds inside a where clause
// fn some_function<T, U>(t: T, u: U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }

pub struct TumblrPost {
    pub username: String,
    pub content: String,
    pub notes: i32,
    pub reply: bool,
}

impl Summary for TumblrPost {
    // fn summarize_author(&self) -> String {
    //     format!("@{}", self.username)
    // }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
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

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Returning types that implement traits
// Here the function returns_summarizable returns a Tweet type
// which implements the Summary trait
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("bazinga"),
        content: String::from("You just got got"),
        reply: false,
        retweet: false,
    }
}

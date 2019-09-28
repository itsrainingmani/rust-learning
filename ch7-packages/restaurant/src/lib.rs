#![warn(dead_code)]
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

fn serve_order() {}

mod back_of_house {

    // Structs can be usable without all their fields being public
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // Associated Function
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Enum fields are all pub by default since it would be annoying to annotate
    // every field with pub
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye Toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change mind about what bread we're going to get
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
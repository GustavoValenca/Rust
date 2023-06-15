// Nested path
use std::{cmp::Ordering, io};

use std::io::{self, Write};
// Same as
// use std::io;
// use std::io::Write;

// Glob Operator
use std::collections::*;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}

        fn seat_at_table(){}
    }

    mod serving {
        fn take_order(){}

        fn serve_order(){}

        fn take_payment(){}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }

    fn cook_order(){}
}

// Re-exporting: external code can now use the path restaurant::hosting::add_to_waitlist() istead of restaurant::front_of_house::hosting::add_to_waitlist()
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // After using the use keyword:
    hosting::add_to_waitlist();
}

fn deliver_order(){}

// Renaming 
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
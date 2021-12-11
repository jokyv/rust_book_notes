// using keyword pub you making it public
// everything is private by default

// bring into scope in a more compact format
use std::{cmp::Ordering, io};
// import module front_of_house
mod front_of_house;

mod back_of_house {
    // enum can be private or public
    // no need to specify if pub the inside of enum
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // here struct is public
    pub struct Breakfast {
        // here toast is public
        pub toast: String,
        // here seasonal_fruit is private!
        seasonal_fruit: String,
    }
}

// PATHS
// use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    // absolute path
    front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // because we use crate::front_of_house::hosting the below is short
    front_of_house::hosting::add_to_waitlist();
}

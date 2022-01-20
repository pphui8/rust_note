// Packages: A Cargo feature that lets you build, test, and share crates
// Crates: A tree of modules that produces a library or executable
// Modules and use: Let you control the organization, scope, and privacy of paths
// Paths: A way of naming an item, such as a struct, function, or module

// create a lib
// cargo new --lib name
// add these to lib.rs
// root of this crate
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use struct and enum
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
}

pub fn eat_at_restaurant() {
    // only if the mod is pub we can use them

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // self or super
    self::front_of_house::hosting::add_to_waitlist(); 

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // enum dosen`t need a function to contruct
    let soup = back_of_house::Appetizer::Soup;
}


// use super
fn serve_order() {}

mod mid_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

// bring the  path in by use

use crate::front_of_house::hosting as host;

// pub use
// `    code use us would use hosting
pub use crate::front_of_house::hosting;

fn Use() {
    host::add_to_waitlist();
    // not recommend to bring function itself in
    use crate::front_of_house::hosting::add_to_waitlist;
    add_to_waitlist();
    // bring struct and enum in
}


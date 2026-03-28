mod back_of_house;
mod front_of_house;

use crate::back_of_house::breakfast::Breakfast;
use crate::back_of_house::appetizer::Appetizer;
pub use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("{:?}", meal.toast);

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;

    add_to_waitlist();

    println!("{:?} {:?}", order1, order2);
}
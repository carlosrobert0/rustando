use std::fmt::Result;
use std::io::Result as IoResult;

mod front_of_house;

use crate::front_of_house::hosting;

fn function1() -> Result { Ok(()) }
fn function2() -> IoResult<()> { Ok(()) }

fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();

    // Relative path
    hosting::add_to_waitlist();
}
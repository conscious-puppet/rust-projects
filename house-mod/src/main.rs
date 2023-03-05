mod house;

use crate::house::{bedroom1, bedroom2};

fn main() {
    println!("Hello, world!");
    println!("{}", bedroom1::is_light_on());
    println!("{}", bedroom2::is_light_on());
    println!("{}", bedroom1::is_neighbour_light_on());
    println!("{}", house::HOUSE_NUMBER);
}

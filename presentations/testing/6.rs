extern crate example;
use example::{is_north, Direction};

#[test]
fn is_north_works() {
    assert!(is_north(Direction::North) == true);
    assert!(is_north(Direction::South) == false);
}
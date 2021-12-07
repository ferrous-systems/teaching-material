
use example::{is_north, Direction};

#[test]
fn is_north_works() {
    assert!(is_north(Direction::North) == true); // <1>
    assert!(is_north(Direction::South) == false);
}

enum Direction { North, South, East, West }

fn is_north(dir: Direction) -> bool {
    match dir {
        Direction::North => true,
        _ => false,
    }
}

#[test]
fn is_north_works() {
    assert!(is_north(Direction::North) == true);
    assert!(is_north(Direction::South) == false);
}
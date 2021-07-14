enum Direction {
    North(u32),
    East(u32),
    South(u32),
    West(u32),
}

fn going_south_or_west(dir: &Direction) -> bool {
    match dir {
        Direction::West(_) | Direction::South(_) => true,
        _ => false,
    }
}

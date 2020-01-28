enum Direction { //<1>
    North(i32), //<2>
    East(i32),
    South(i32),
    West(i32),
}

fn going_west(dir: &Direction) -> bool {
    match dir { //<3>
        Direction::West(_) => true, //<4>
        _ => false
    }
}

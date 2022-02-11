'outer: for i in 0..10 {
    loop {
        if i < 5 {
            continue 'outer;
        } else {
            break 'outer;
        }
    }
}
fn main() {
    'outer: for i in 0..10 {
        for j in 0..5 {
            if i + j > 10 {
                break 'outer;
            }
            if j % 2 == 1 {
                continue;
            }
            println!("Sum: {}+{}={}", i, j, i + j);
        }
    }
}

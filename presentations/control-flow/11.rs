fn main() {
    let mut i = 0;

    while !(i > 100) {
        i += 1;
    }

    let mut iter = vec![1,2,3].into_iter();

    while let Some(i) = iter.next() {
        println!("number: {}", i);
    }
}

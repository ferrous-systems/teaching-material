trait VecExt {
    fn magic_number(&self) -> usize;
}

impl<T> VecExt for Vec<T> {
    fn magic_number(&self) -> usize {
        42
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("Magic Number = {}", v.magic_number());
}

fn main() {
    let num = 5;

    let owns_num = move |x: i32| x + num;
}
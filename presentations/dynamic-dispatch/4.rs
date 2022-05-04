fn factory() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    Box::new(move |x| x + num)
}

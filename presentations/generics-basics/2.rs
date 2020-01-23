enum Either<T, X> {
    Left(T),
    Right(X)
}

fn main() {
    let alternative: Either<i32, f64> = Either::Left(123);
}
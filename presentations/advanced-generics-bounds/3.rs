trait Centered {
    fn center(&self) -> (i32, i32);
}

impl<X,T> Distance<X> for T
    where T: Centered,
          X: Centered {
}
#[inline]
pub fn first_layer_of_indirection<P: AsRef<Path>>(p: P) {
    let path = p.as_ref();
    something_using_the_path(path);
}

fn something_using_the_path(p: Path) {
    //....
}
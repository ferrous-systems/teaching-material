pub fn first_layer_of_indirection<P: AsRef<Path>>(p: P) {
    something_using_the_path(p);
}

fn something_using_the_path<P: AsRef<Path>>(p: P) {
    //....
}
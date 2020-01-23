fn main() {
    let some_result = this_can_fail(true);
    // Only done if `some_result` is an `Ok` Variant.
    let mapped_result = some_result.map(|val| val.len());
    println!("{:?}", mapped_result);
}
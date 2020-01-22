fn main() {
    let maybe_value = Some(2);
    match maybe_value {
        Some(value) if value == 2 => {
            // ...
        }
        Some(value) => {
            // ...
        },
        None => {
            // ...
        },
    }
}
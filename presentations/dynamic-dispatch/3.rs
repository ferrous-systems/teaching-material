use std::fmt::Debug;
use std::any::Any;

// Logger function for any type that implements Debug.
fn log<T: Any + Debug>(value: &T) {
    let value_any = value as &Any;

    // try to convert our value to a String.  If successful, we want to
    // output the String's length as well as its value.  If not, it's a
    // different type: just print it out unadorned.
    match value_any.downcast_ref::<String>() {
        Some(as_string) => {
            println!("String ({}): {}", as_string.len(), as_string);
        }
        None => {
            println!("{:?}", value);
        }
    }
}

fn main() {
    log(&String::from("String here"));
    log(&1)
}
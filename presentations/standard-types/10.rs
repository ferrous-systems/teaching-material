use std::collections::HashMap;

fn main() {
    let mut kv_store = HashMap::new();
    kv_store.insert("key", true);
    println!("{}", kv_store.get("key"));
}
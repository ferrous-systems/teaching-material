use std::collections::HashMap;

fn main() {
    let mut kv_store = HashMap::new();
    let mut value = kv_store.entry("key").or_insert(true);
    *value = false;
}
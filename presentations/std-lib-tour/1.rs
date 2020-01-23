use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("foo").or_insert(1);
    map.entry("bar").or_insert_with(|| {
        let mut value = 1;
        value += 1;
        value
    });
    println!("{:?}", map);
}
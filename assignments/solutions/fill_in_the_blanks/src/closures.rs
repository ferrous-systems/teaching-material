fn main() {
    // Recap: Anatomy of a closure
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //
    let rummage = |element| {
        // 👈  input parameters MAY omit type annotation
        //     if type can be inferred. The input parameter
        //     is also referred to as the "captured value".
        println!("found: {}", element);
        // 👈  no return statement: closures *can* return values,
        //     but this one doesn't
    }; // 👈  {}s are only needed for multi-line closures

    let haystack = vec!["hay", "hay", "hay", "needle", "hay", "hay"];

    // REFERENCING CLOSURES
    //=====================
    // *By default*, closures capture variables by reference. When given a choice, this is the
    // behaviour inferred by the compiler.

    haystack.iter().for_each(rummage);
    // note that we can directly borrow the contents of `haystack` after using the `rummage`
    // closure, because it only borrowed the elements (and is now done)
    println!("top of the haystack: {}", haystack[0]);

    // MUTATING CLOSURES
    //==================
    // Closures can mutate the variables they are capturing

    // 👀  Closures can be used as function arguments.

    // ✅ TODO: remove all the hay from `haystack` by checking whether `key` is a needle
    let mut haystack_clone = haystack.clone();
    haystack_clone.retain(|key| *key == "needle");
    println!("look, I found the needle: {:?}", haystack_clone);

    // 👀  a common use case for closures is to transform collections
    //     using e.g. `map()` and `filter()`.

    // ✅ TODO: use `map()` to convert every "hay" in the haystack to a "🌾"

    let emoji_haystack: Vec<_> = haystack
        .into_iter()
        .filter(|element| *element == "hay")
        .map(|item| item.replace("hay", "🌾"))
        .collect();

    println!("emoji haystack: {:?}", emoji_haystack);

    // ✅  TODO: try uncommenting this line. What happens when you re-compile and why?
    // println!("haystack: {:?}", haystack);

    // ✅  Bonus Task: re-implement the creation of `emoj_haystack` using `filter_map()`
    let filter_map_haystack: Vec<_> = vec!["hay", "hay", "hay", "needle", "hay", "hay"]
        .into_iter()
        .filter_map(|element| match element {
            "hay" => Some("🌾"),
            _ => None,
        })
        .collect();
    println!("filter_map_haystack: {:?}", filter_map_haystack);
}

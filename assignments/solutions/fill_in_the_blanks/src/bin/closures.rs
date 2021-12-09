#[rustfmt::skip]
fn main(){
    // RECAP: ANATOMY OF A CLOSURE
    //============================
    //
    let outside = "it's raining!";
    let rummage = | element | { // 👈 input parameters MAY omit type annotation
                                // if the type can be inferred.
        
        println!("found: {}", element);
        println!("meanwhile, the weather: {}", outside); //  👈 closures can
        // refer to ("capture") the surrounding environment.
        // *By default*, the environment is borrowed (shared or mutable).
        // When given a choice, this is the behaviour picked by the compiler: shared > mutable > owned.
        // For further details, see https://doc.rust-lang.org/reference/types/closure.html#capture-modes

        // 👈  no return statement: just like functions or blocks, closures *can* return values,
        // but this one doesn't - hence it implictly returns the empty tuple `()`.
    };  // 👈  {}s are only needed for multi-line closures

    let haystack = vec!["hay", "hay", "hay", "needle", "hay", "hay"];

    // 👀  Closures can be used as function arguments.
    haystack.iter().for_each(rummage);

    // since we captured our environment in a shared (immutable) fashion, we're free to run this closure again:
    haystack.iter().for_each(rummage);
    

    // MUTATING CLOSURES
    //==================
    // Closures can mutate the variables they are capturing

    // ✅ TODO: remove all the hay from `haystack` by checking whether `key` is a needle
    // ✅ TODO: as a side effect, count the hay
    let mut haystack_clone = haystack.clone();
    let mut hay_count = 0;
    haystack_clone.retain(|key| {
        let is_needle = *key == "needle";
        if !is_needle {
            hay_count += 1;
        }
        is_needle
    });
    println!(
        "look, I found the needle amid {} pieces of hay: {:?}",
        hay_count, haystack_clone
    );

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

    // ✅  Bonus Task: re-implement the creation of `emoji_haystack` using `filter_map()`
    let haystack = vec!["hay", "hay", "hay", "needle", "hay", "hay"];
    let filter_map_haystack: Vec<_> = haystack
        .into_iter()
        .filter_map(|element| match element {
            "hay" => Some("🌾"),
            _ => None,
        })
        .collect();
    println!("filter_map_haystack: {:?}", filter_map_haystack);
}

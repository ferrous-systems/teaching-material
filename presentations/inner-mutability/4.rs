fn main() {
    let post = Post {
        content: String::from("Blah"),
        ..Post::default()
    };
    (0..5).for_each(|_| post.view());
    println!("{:?}", post);
}

#[derive(Debug, Default)]
struct Post {
    content: String,
    viewed_times: Cell<usize>,
}

impl Post {
    fn view(&self) {
        // Note how we are making a copy, then replacing the original.
        let current_views = self.viewed_times.get();
        self.viewed_times.set(current_views + 1);
    }
}

use std::cell::Cell;
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
    viewed_times: RefCell<usize>,
}

impl Post {
    fn view(&self) {
        // Note how we're mutating a value.
        *self.viewed_times.borrow_mut() += 1;
    }
}

use std::cell::RefCell;
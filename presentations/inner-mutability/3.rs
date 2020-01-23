fn main() {
    // We need to make the entire struct mutable!
    let mut post = Post {
        content: String::from("Blah"),
        ..Post::default()
    };
    (0..5).for_each(|_| post.view());
    println!("{:?}", post);
}

// From before

#[derive(Debug, Default)]
struct Post {
    content: String,
    viewed_times: usize,
}

impl Post {
    // `mut` is a problem here!
    fn view(&mut self) {
        self.viewed_times += 1;
    }
}
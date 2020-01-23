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
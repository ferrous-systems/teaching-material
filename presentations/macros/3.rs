macro_rules! email {
    ($user:expr => $domain:expr) => {
        format!("{}@{}", $user, $domain);
    }
}

fn main() {
    let address = email!("me" => "example.org");
    println!("{}", address);
}
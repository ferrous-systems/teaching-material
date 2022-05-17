$ cargo new hello-world
$ cd hello-world
$ cat src/main.rs
fn main() {
    println!("Hello, world!");
}
$ cargo build
   Compiling hello-world v0.1.0 (file:///Users/skade/Code/rust/scratchpad/hello-world)
    Finished debug [unoptimized + debuginfo] target(s) in 0.35 secs
$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello-world`
Hello, world!

use std::env;
use std::fs::File;
use std::io::prelude::*;

// Adapted from https://haggainuchi.com/nestedparens.html
// Skip this function, nothing to do.
fn nested(n: i32) -> Vec<String> {
    if n == 0 {
        vec![String::new()]
    } else {
        let mut parens = Vec::new();
        for i in 0..n {
            for a in nested(i) {
                for b in nested(n - 1 - i) {
                    parens.push("(".to_owned() + &a + ")" + &b);
                }
            }
        }
        parens
    }
}

fn main() -> std::io::Result<()> {
    // Collect an i32 input from CLI
    let args: Vec<String> = env::args().collect();
    let input = if args.len() != 2 {
        panic!("Please only supply a single i32 as CLI arg. e.g., `cargo run 10`");
    } else {
        let num = args[1].parse::<i32>().unwrap();
        if num < 1 || 15 < num {
            panic!("Please only supply an i32 between 2 and 13 as an arg")
        }
        num
    };

    // Create a file name
    let parens_vec = nested(input);
    let name = format!("bench_{}_length_{}.txt", &input, &parens_vec.len());

    // Skip if File already exists,
    // Otherwise write the Vec<String> into the file, one element per line
    if std::path::Path::new(&name).exists() {
        panic!("File already exists. Exiting.");
    }
    let mut file = File::create(&name).expect("File already existed!");
    for f in &parens_vec {
        writeln!(file, "{}", f)?;
    }
    println!("File {} with length {} written.", &name, &parens_vec.len());
    Ok(())
}

// `for loop` solution
// On LeetCode, this code obtains:
// Runtime: 1 ms, faster than 53.49% of Rust online submissions for Maximum Nesting Depth of the Parentheses.
// Memory Usage: 2.2 MB, less than 23.26% of Rust online submissions for Maximum Nesting Depth of the Parentheses.
fn max_depth1(s: String) -> i32 {
    let mut max_count = 0;
    let mut count = 0;
    for c in s.chars() {
        if c == '(' {
            count += 1;
            if count > max_count {
                max_count = count;
            }
        } else if c == ')' {
            count -= 1;
        }
    }
    max_count
}

// `iterator` solution, First Pass Attempt!
// On LeetCode, this code obtains:
// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Maximum Nesting Depth of the Parentheses.
// Memory Usage: 1.9 MB, less than 97.67% of Rust online submissions for Maximum Nesting Depth of the Parentheses.
pub fn max_depth2(s: String) -> i32 {
    let mut max_count = 0;
    let mut count = 0;
    s.chars().for_each(|c| {
        if c == '(' {
            count += 1;
            if count > max_count {
                max_count = count;
            }
        } else if c == ')' {
            count -= 1;
        };
    });
    max_count
}
#[test]
fn max_depth1_works() {
    assert_eq!(max_depth1(String::from("()()()")), 1);
    assert_eq!(max_depth1(String::from("((()))")), 3);
    assert_eq!(nested(10).into_iter().map(max_depth1).max().unwrap(), 10);
}

#[test]
fn max_depth2_works() {
    assert_eq!(max_depth2(String::from("()()()")), 1);
    assert_eq!(max_depth2(String::from("((()))")), 3);
    assert_eq!(nested(10).into_iter().map(max_depth2).max().unwrap(), 10);
}

// Pull in eval and parse from the lib.rs/crate.
use calc::prelude::*;
use std::io;

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(ParseError),
    Eval(EvalError),
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<ParseError> for CliError {
    fn from(err: ParseError) -> CliError {
        CliError::Parse(err)
    }
}

impl From<EvalError> for CliError {
    fn from(err: EvalError) -> CliError {
        CliError::Eval(err)
    }
}

fn main() {
    // "REPL"
    // Read, Evaluate, Print, Loop
    loop {
        match process_one() {
            Ok(ShouldContinue::Yes) => {}
            Ok(ShouldContinue::No) => {
                break;
            }
            Err(CliError::Io(e)) => {
                println!("Fatal Error: {:?}", e);
                break;
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    // Basically how a "for" loop works in Rust.
    // loop {
    //     match iter.next() {
    //         Some(data) => {
    //             // THIS IS THE BODY OF THE FOR LOOP
    //         }
    //         None => break,
    //     }
    // }
}

enum ShouldContinue {
    Yes,
    No,
}

// NOTE: These two approaches are equivalent. The former is more concise.
//
// let n_bytes_read = io::stdin()
//     .read_line(&mut buf)
//     .map_err(CliError::Io)?;
//
// // Manually is:
// let n_bytes_read = match io::stdin().read_line(&mut buf) {
//     Ok(bytes) => bytes,
//     Err(e) => {
//         // This is the `map_err` part, the rest of the match statement
//         // is the `?` part (for early return)
//         return Err(CliError::Io(e))
//     }
// };

fn process_one() -> Result<ShouldContinue, CliError> {
    // Wait for standard input for a line
    let mut buf = String::new();

    // Attempt to read a line from standard in (blocking)
    let n_bytes_read = io::stdin().read_line(&mut buf)?;

    // If we have an empty line, skip
    if n_bytes_read == 0 {
        return Ok(ShouldContinue::Yes);
    }

    // Process user exit command
    if "exit" == buf.trim() {
        println!("Exit!");
        return Ok(ShouldContinue::No);
    }

    // parse the line
    let parsed = parse(buf.trim())?;

    // Evaluate the line
    let evaled = eval(&parsed)?;

    // print the result
    println!("{}", evaled);

    // We succeeded, move on to the next iteration
    Ok(ShouldContinue::Yes)
}

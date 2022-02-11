# 2021-12-09

## Fill in the blanks: Closures
- clarify closure environment capturing
- update solution

# 2021-12-07
## Rust 2015
- remove `extern crate` in `*.rs`
- cosmetics followup: fold multiple empty lines in `*.rs`

## Lifetimes
- convert from Rust 2015 (add `impl` in fn signature)
- fix typo in variable name
- use non-copy value to illustrate lifetime issues (`String` instead of `i32`)
- remove german-ism ("Lifetime-Elision")

# 2021-12-03
- reworked FFI chapter:
  - add `/examples/use_{rust,c}_in_{c,rust}` projects
  - add speaker notes as comments in `slides.adoc`
  - starting with using Rust code in a C program
  - replace ImageMagick and weechat examples with simple addition math

# 2021-12-01

## Misc
- add unlinked slide decks to index under "Other"

# 2021-12-01

## Semver Codealong
- add `/semver-codealong` code snippets/instructor notes for semver trail (WIP)

## Misc
- add `assignments/_templates/concurrency` for a simple `Send+Sync` exercise
- rename `assignments/preliminary/` to `assignments/_preliminary` 

# 2021-11-03

## Control Flow
- change `Option` example from `into_iter()` rabbit hole explosion to checked addition
- add `break`, `continue` + labels
- add explanation of `for` machinery
- clarify `std::env::args().nth(n)` example
- wording

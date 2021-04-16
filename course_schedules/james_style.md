> note: this is based on Aleksey's schedule: https://gist.github.com/matklad/5f930307b62366189b074d3828af5488

6 half-days, 4 hours per day.
I did only one small break in the middle of the day, this is waaay too few breaks, but I was pretty bad at timing breaks, as there's always just one more thing you want to tell about.
Notes are written after-the-fact, I probably covered more things than specified here, but I don't rememner them :-)

== Day 1

.Prep
* log in to zoom a couple of minutes before the start, to check mic and audio
* pin tab with https://ferrous-systems.github.io/teaching-material/index.html
* pin tab with hackmd: https://hackmd.io/@matklad/ryq1uNBcw/edit
* (create a saved session in browser to quickly restore the tabs)

=== Round of introductions

Getting to know names and backgrounds of folks.
This is a mixed group: some folks don't know Rust, some are already pretty good at it.

=== Overview

Slides:: https://ferrous-systems.github.io/teaching-material/overview.html
Digression:: language stability

=== Toolchain

Sides:: https://ferrous-systems.github.io/teaching-material/installation.html
Digression:: semver
Exercise::
+
--
Getting the feet wet!
Compiling a hello-world program

[source]
----
$ rustc hello.rs
$ cargo new hello-world
----
--

=== Break

=== Primitive Types

Slides:: https://ferrous-systems.github.io/teaching-material/basic-types.html

Digression::
+
--
* Longish degression about primitives API (wrapping, unchecked, popcount, etc).
Relevant for folks who do C things.
* Introduce `mem::size_of` (I used it heavily throughout the course).
* `*const T` can be wide
--

Exercise::
--
1. Trigger overflow error in the code.

2. See the difference in debug profile vs release profile.

3. Read cargo docs to enable overflow checks
--

=== Compound Types

Slides:: https://ferrous-systems.github.io/teaching-material/compound-types.html
Digression:: zero-sized types

=== Control Flow I

Slides:: https://ferrous-systems.github.io/teaching-material/control-flow.html, not including loops

== Day 2

=== Control Flow II

Slides:: https://ferrous-systems.github.io/teaching-material/control-flow.html, loops
Digression:: definitive assignment, a touch of `!` type, semicolons & return, break with value
Exercise:: https://ferrous-systems.github.io/teaching-material/assignments/fizzbuzz.html
Digression:: `match ... { (true, false) => ... }` vs chain of ifs, exhaustiveness, `_`.

=== Ownership & Borrowing

Slides:: https://ferrous-systems.github.io/teaching-material/ownership-borrowing-in-brief.html
Digression:: set the scene up for "`shared & exclusive`" terminology later in the course, by admitting that everything is a lie in this example (we should come with a better example than a file prob)

=== Break

=== Practice

Exercise:: https://ferrous-systems.github.io/teaching-material/assignments/result-option-assignment.html
Bonus Points:: re-use the same buffer for all strings, explain why Rust's API is shaped the way it is (to allow buffer re-use).
Digression:: preludes, allocating a new strings vs re-using existing `&mut buf`.

First complex exercise, spend a lot of time on this one (in general, I severely underestimated the time folks need to go through the exercises).

== Basic Traits

Slides:: https://ferrous-systems.github.io/teaching-material/traits.html (without associated types).

Prob makes sense to push this to __before__ the files exercise?
IO requires traits :-(

== Day 3

Starting roughly from here, I reduced the amount of me explaining the slides, and increase the share of digressions/coding.
At this point, there's enough vocabulary to have interesting discussions.

=== Ownership & Borrowing

Refresh an important topic from yesterday.

Mostly a

Digression::
--
Variations on this example:

[source,rust]
----
let mut xs = vec![1, 2, 3];
let x = &xs[0];
xs.push(92);
println!("{}", *x);
----

Compare with {cpp}, explain why even `xs.swap(0, 1)` may be problematic.
--

=== Error Handling

Slides:: https://ferrous-systems.github.io/teaching-material/error-handling.html#
Digression:: `anyhow` and similar ecosystem traits

=== Drop, Panic, Abort

Slides:: https://ferrous-systems.github.io/teaching-material/drop-panic-abort.html

=== Break

=== DurableFile

Exercise:: https://ferrous-systems.github.io/teaching-material/assignments/durable-file.html
Digression:: unit tests, integration tests, doctests, compilefail tests

=== Strings

Slides:: https://ferrous-systems.github.io/teaching-material/strings.html
Digression:: more context about Os and C strings, PathBuf and Path. Pitfalls of AsRef polymorphism, first glance at monomorphization.

== Day 4

=== Calc Scaffold

Exercise:: https://ferrous-systems.github.io/teaching-material/assignments/calc.html (steps 1 & 2)
Digression:: Size of recursive enums, enum variants as types

=== Calc Logic

Exercise:: Steps 3, 4, 5, and 6 from calc
Digression:: very thorough review of existing solutions, sharing code between match arms, designing `enum` to reduce copy-paste, basic info about tests.

=== Break

=== Generics

Slides:: https://ferrous-systems.github.io/teaching-material/generics-basics.html + https://ferrous-systems.github.io/teaching-material/advanced-generics-bounds.html
Digression:: Monomorphisation, recursive function producing an infinitely large type
Exercise:: https://ferrous-systems.github.io/teaching-material/assignments/calc.html#_idiomatic_api
Digression:: return-type polymorphism, parse and FromStr


== Day 5

=== Modules & Crates

Slides:: https://ferrous-systems.github.io/teaching-material/imports-modules-and-visibility.html
Digression:: DAG of crates, no single global namespace, Cargo strategy for resolving version conflicts (duplication of majors), contrast with C model.
Digression:: Importance of visibilty, jump from `pub(crate)` to `pub`.
Exercise:: https://ferrous-systems.github.io/teaching-material/assignments/calc.html#_modularization
Digression:: "`read zero bytes`" condition in std's API

=== Break

=== FFI

Digression:: show how to wrap calc into a C API

=== Closures

Slides:: https://ferrous-systems.github.io/teaching-material/closures.html
Digression:: why size of the closure is zero? Closures vs function poiners.

== Day 6

=== Iterators

Slides:: https://ferrous-systems.github.io/teaching-material/iterators.html#/
Digression:: reading the sources of standard library, understanding why size_of of a chain of maps and filters is the same as for the basic iterator.
Digression:: collect / sum

=== Threads

Digression:: show basic thread's API, `Arc`, `Mutex`, atomics
Exercise:: https://ferrous-systems.github.io/teaching-material/assignments/calc.html#_naive_multithreading
Bonus Points:: implement thread-safe buffer pool.

=== Ownershipt and Borrowing

Digression:: Give the correct aliasing intuition for O&B system, compare with `__restrict` and TBAA of C.
Really stress shared&exclusive terminology


notes: https://hackmd.io/Ihed_aFcQlOrl7H8bjLj4w
# Standard Types
[Table of Contents](toc/english.html)

---

There several pervasive types in Rust.

They leverage the powerful type system to accomplish fundamental tasks.

---

## Overview

-   `Option<T>` - Removes the need for a `null` primitive.
-   `Result<T,E>` - Removes the need for exceptions.
-   `Vec<T>` - Growable arrays.
-   `HashMap<K,V>` - Key value storage.

---

## `Option<T>`

<pre><code data-source="chapters/shared/code/standard-types/1.rs" data-trim="hljs rust"></code></pre>
Options are wrapper types, and need to be unwrapped to be used.

---

## `Option<T>`

Any function which does not always return a value returns an `Option<T>`.

<pre><code data-source="chapters/shared/code/standard-types/2.rs" data-trim="hljs rust"></code></pre>

---

## `Option<T>`: Benefit

The programmer *always* knows where a `None` may appear, and is able to decide how the situation should be handled.

This characteristic helps *remove mystery* from the coding process, and aids in confidence.

---

## `Option<T>`: Unwrapping

`unwrap()` will panic the application if the value is `None`.

This is only recommended in testing and prototyping.

<pre><code data-source="chapters/shared/code/standard-types/3.rs" data-trim="hljs rust"></code></pre>

---

## `Option<T>`: Safety

`match` is one of several ways to safety work with `Option`s.

<pre><code data-source="chapters/shared/code/standard-types/4.rs" data-trim="hljs rust"></code></pre>
No matter what the value of `maybe_a_value`, the program will never crash.

---

## `Option<T>`: Questions

Does this type completely remove the need for a `null` primitive?

What are the benefits?

---

## `Result<T,E>`

<pre><code data-source="chapters/shared/code/standard-types/5.rs" data-trim="hljs rust"></code></pre>
Results are wrapper types which either contain the successful value, or the error value.

---

## `Result<T,E>`: Using

Results can be handled via `unwrap()` just like `Option` types, and can be handled in the same ways.

<pre><code data-source="chapters/shared/code/standard-types/6.rs" data-trim="hljs rust"></code></pre>
Handling complex error scenarios will be addressed in a later chapter.

---

## `Result<T,E>`: Questions

Does this type completely remove the need for exceptions?

What are the benefits?

---

## `Vec<T>`

Owned, mutable, growable arrays. Located on the heap.

<pre><code data-source="chapters/shared/code/standard-types/7.rs" data-trim="hljs rust"></code></pre>

---

## `Vec<T>`: Creation

Create with `Vec::new()` or the `vec![]` macro.

<pre><code data-source="chapters/shared/code/standard-types/8.rs" data-trim="hljs rust"></code></pre>

---

## `Vec<T>`: As a Slice

`Vec<T>` implements `Deref<Target=[T]`, so it can be easily used as a slice.

<pre><code data-source="chapters/shared/code/standard-types/9.rs" data-trim="hljs rust"></code></pre>

---

## `HashMap<K,V>`

HashMaps are key value stores. Keys must implement `Hash`.

<pre><code data-source="chapters/shared/code/standard-types/10.rs" data-trim="hljs rust"></code></pre>

---

## `HashMap<K,V>`: `entry()`

Manipulate a key's cooresponding entry in place.

<pre><code data-source="chapters/shared/code/standard-types/11.rs" data-trim="hljs rust"></code></pre>


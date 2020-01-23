# Iterators
[Table of Contents](toc/english.html)

---

Rusts iterators are:

-   Lazy
-   Pervasive
-   Potentially infinite

---

## Where Do They Come From?

-   Collections like `Vec<T>` have an `iter()` function which yields an iterator.
-   Things like `std::net::TcpListener` which provides an iterator of `TcpStream`s via `incoming()`.
-   Iterators can be implemented on other structures as well.

---

## Common Uses

---

## `next()`

Iterators can be manually progressed:

<pre><code data-source="chapters/shared/code/iterators/1.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## `map()`

Transform items as they are evaluated:

<pre><code data-source="chapters/shared/code/iterators/2.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## `filter()`

Filter out unwanted values, skipping further computation on them:

<pre><code data-source="chapters/shared/code/iterators/3.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## `fold()`

Reduce a sequence of values down to a single value:

<pre><code data-source="chapters/shared/code/iterators/4.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## `cycle()` & `take()`

Cause iterators to repeat, and end early:

<pre><code data-source="chapters/shared/code/iterators/5.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## `zip()` & `unzip()`

Join iterators, and split them apart again:

<pre><code data-source="chapters/shared/code/iterators/6.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## `max()` & `min()`

<pre><code data-source="chapters/shared/code/iterators/7.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## Gotcha

This doesn't work:

<pre><code data-source="chapters/shared/code/iterators/8.rs" data-trim="hljs rust" class="lang-rust"></code></pre>
Why? (Think about scoping & ownership!)

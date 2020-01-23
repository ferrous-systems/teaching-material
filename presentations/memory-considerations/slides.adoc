# Memory considerations
[Table of Contents](toc/english.html)

---

`size_of` and `size_of_val` help to inspect the size of a type (in bytes).

<pre><code data-source="chapters/shared/code/memory-considerations/1.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## Sizedness

All values in Rust have a fixed or a flexible-sized memory size. The language expresses that through the marker trait `Sized`. `Sized` types have a size known at compile time.

Unsized types *must* be addressed through indirection (e.g. through borrows).

---

## Size of Structs

Structs have the size of their contained values.

<pre><code data-source="chapters/shared/code/memory-considerations/2.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## References

References have the size of 1 `usize`. The same is true for simple boxes.

<pre><code data-source="chapters/shared/code/memory-considerations/3.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## Trait Objects

Trait objects need 2 `usize`.

<pre><code data-source="chapters/shared/code/memory-considerations/4.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## Slices

Slices need 2 usize. The same is true for their boxes.

<pre><code data-source="chapters/shared/code/memory-considerations/5.output" data-trim="hljs output" class="lang-rust"></code></pre>

---

## Vectors

Vectors need 3 usize, to additionally store their capacity.

<pre><code data-source="chapters/shared/code/memory-considerations/6.output" data-trim="hljs output" class="lang-rust"></code></pre>

---

## `&str`, `Box<str>`, `String`

They behave the same as slices and vectors.

---

## Enums

Enums need the size of their largest value, 1 byte for a discriminant, plus padding.

<pre><code data-source="chapters/shared/code/memory-considerations/7.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

If 0 is not a legal value, Option uses this as a the `None`-Case.

There is currently no stable way to communicate this to the compiler.


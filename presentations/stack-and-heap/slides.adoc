# Stack and Heap
[Table of Contents](toc/english.html)

---

Rust defaults to allocation on the stack

---

## Stack Allocation

<pre><code data-source="chapters/shared/code/stack-and-heap/1.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## Box

Heap allocation is represented by the type `Box`.

<pre><code data-source="chapters/shared/code/stack-and-heap/2.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## Ownership and Borrowing

`Box` is owned, but you can borrow the contained values.

<pre><code data-source="chapters/shared/code/stack-and-heap/3.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## Other heap allocations

Other types might allocate on the heap, most notably `Vec` and `String`.

---

## Placement in

It is currently *not* possible to allocate values at a self-chosen location. The missing feature is called "placement in".

[Detailed discussion here](https://internals.rust-lang.org/t/lang-team-minutes-feature-status-report-placement-in-and-box/4646)

---

In most cases, LLVM already optimizes the stack allocation and the subsequent move to the heap to a direct heap allocation.
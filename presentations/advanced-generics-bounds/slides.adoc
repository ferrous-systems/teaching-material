# Advanced Generics: Bounds
[Table of Contents](toc/english.html)

---

Sometimes, we want to constrain a type to certain aspects, while still writing generic code.

To achieve this, we can constrain type parameters.

---

This can happen at any point where type parameters are used.

---

Example: `println("{:?}")` requires Debug
 
<pre><code data-source="chapters/shared/code/advanced-generics-bounds/1.rs" data-trim="hljs rust"></code></pre>

---

Example: A generic Struct that requires inner values to implement `Debug`

<pre><code data-source="chapters/shared/code/advanced-generics-bounds/2.rs" data-trim="hljs rust"></code></pre>

---

Bounds can also be expressed for implementation targets:

<pre><code data-source="chapters/shared/code/advanced-generics-bounds/3.rs" data-trim="hljs rust"></code></pre>

---

Traits can also directly require prerequisites:

<pre><code data-source="chapters/shared/code/advanced-generics-bounds/4.rs" data-trim="hljs rust"></code></pre>

---

Rust does not allow negative Bounds (Trait A and *not* Trait B)

---

## Exception: `Sized`

If not specified otherwise, all type parameters carry the bound `Sized` (the type has a statically known memory size). This can be suppressed by using the bound `?Sized`.

<pre><code data-source="chapters/shared/code/advanced-generics-bounds/5.rs" data-trim="hljs rust"></code></pre>

---

This has ergonomic reasons, as passing types by value is common and requires a known size.

---

Bounds can be used everywhere, which can be used to de-facto constrain types at the call site.

<pre><code data-source="chapters/shared/code/advanced-generics-bounds/6.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

This can be very practical, as this allows expressing different bounds during construction and at call sites.

---

Bounds are very common in conversion functions.

<pre><code data-source="chapters/shared/code/advanced-generics-bounds/7.rs" data-trim="hljs rust"></code></pre>

---

## Generic implementations

Bounds can be used to constrain the target of an implementation.

<pre><code data-source="chapters/shared/code/advanced-generics-bounds/8.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## Trait Inheritance

Traits can also request the implementation of other traits and declare default implementations for methods relying on that information.

<pre><code data-source="chapters/shared/code/advanced-generics-bounds/9.rs" data-trim="hljs rust" class="lang-rust"></code></pre>


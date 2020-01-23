# Send & Sync
[Table of Contents](toc/english.html)

---

There are two special traits in Rust for concurrency semantics.

-   `Send` marks a structure safe to *send* between threads.
-   `Sync` marks a structure safe to *share* between threads.
    -   (`&T` is `Send`)

---

These traits are what Rust uses to prevent data races.

They are *automatically derived* for all types if appropriate.

---

## Automatically Derived

<pre><code data-source="chapters/shared/code/send-and-sync/1.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

There are some notable types which are not `Send` or `Sync`.

Such as `Rc`, raw pointers, and `UnsafeCell`.

---

## Example: `Rc`

<pre><code data-source="chapters/shared/code/send-and-sync/2.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## Example: `Rc`

<pre><code data-source="chapters/shared/code/send-and-sync/3.output" data-trim="hljs output"></code></pre>

---

## Implementing

It's possible to add the implementation of `Send` and `Sync` to a type.

<pre><code data-source="chapters/shared/code/send-and-sync/4.rs" data-trim="hljs rust"></code></pre>
In these cases, the task of thread safety is left to the implementor.

---

## Relationships

If a type implements both `Sync` and `Copy` then it can also implement `Send`.

---

## Relationships

A type `&T` can implement `Send` if the type `T` also implements `Sync`.

<pre><code data-source="chapters/shared/code/send-and-sync/5.rs" data-trim="hljs rust"></code></pre>

---

## Relationships

A type `&mut T` can implement `Send` if the type `T` also implements `Send`.

<pre><code data-source="chapters/shared/code/send-and-sync/6.rs" data-trim="hljs rust"></code></pre>

---

## Consequences

What are the consequences of having `Send` and `Sync`?

---

## Consequences

Carrying this information at the type system level allows driving data race bugs down to a *compile time* level.

Preventing this error class from reaching production systems.

`Send` and `Sync` are independent of the choice of concurrency (async, threaded, etc.).


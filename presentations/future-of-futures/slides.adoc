# The Future of Futures
[Table of Contents](toc/english.html)

---

Hurry up and wait

`async` & `await`

---

Future = asynchronous computation

* Network IO
* Message from another thread or computer
* Chained operations

---

Rust has no implicit runtime event loop (like i.e. node.js)

---

## Layers of abstractions

---

`Future<T>` and `Poll<T>`

Part of `libcore` and `libstd`

---

`futures-rs` abstraction crate on top of those

---

`tokio` abstraction crate provides event-loop

---

## It's all...not stable

Requires nightly and `futures_api` feature-flag

---

<pre><code data-source="chapters/shared/code/future-futures/1.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

<pre><code data-source="chapters/shared/code/future-futures/2.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

Most futures won't be ready immediately.

Instead return `Poll::Pending` until ready.

`poll()` future from event-loop.

---

![](img/futures-rs-logo.svg)

`futures-rs` to the rescue

---

There's `oneshot`

<pre><code data-source="chapters/shared/code/future-futures/3.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

And combinators on futures

<pre><code data-source="chapters/shared/code/future-futures/4.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## Explicit runtime

![](img/tokio.jpg)

`tokio-rs` to the rescue

---

`tokio` provides an async-executor and runtime

<pre><code data-source="chapters/shared/code/future-futures/5.rs" data-trim="hljs rust" class="lang-rust"></code></pre>


---

## Future of `futures`

<br/>

* Two new keywords
* Write code that looks synchronous
  * Get async for free*


<small>* (ish)</small>

---

<pre><code data-source="chapters/shared/code/future-futures/6.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## I want that!

---

## Timeline

* RFC 2394 (tracking issue #50547)
* Implemented in the compiler
* Several blocking issues (#51719, #53249, #53259, ...)
* _Not_ included in Rust 2018
  * But soon™ (~~late 2018~~ 2019!)
# Property Testing
[Table of Contents](toc/english.html)

---

## This is your brain

-   Everything we know is subject to bias
-   Everything we build reflects these biases

---

## Problem:

Our code reflects our biases, our tests are often biased similarly

---

## Solution:

Don't write tests

---

## Solution:

Write expectations

---

-   Have the machine generate random test cases
-   Make beliefs explicit, force them to pay rent

---

This is called property testing

---

## Crate: **proptest**

<pre><code data-source="chapters/shared/code/proptest/1.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## Crate: **proptest**

<pre><code data-source="chapters/shared/code/proptest/2.txt" data-trim="hljs bash" class="lang-bash"></code></pre>

---

## Crate: **proptest**

<pre><code data-source="chapters/shared/code/proptest/3.txt" data-trim="hljs bash" class="lang-bash"></code></pre>

---

Wonderful for testing codecs, serialization,
compression, or any set of operations that
should retain equality.

<pre><code data-source="chapters/shared/code/proptest/4.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

It's easy to generate more structured input, too

<pre><code data-source="chapters/shared/code/proptest/5.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## Configuration is a great target

<pre><code data-source="chapters/shared/code/proptest/6.rs" data-trim="hljs rust" class="lang-rust"></code></pre>

---

## Miscellaneous Tips

-   Isolate business logic from IO concerns
-   Use assert! and debug_assert! on non-trivial things! this makes our "fuzzers" extremely effective
-   Try not to use unwrap() everywhere, at least use expect("helpful message") to speed up debugging
-   When propagating errors, include context that helps you get back to the root

---

Try it out!

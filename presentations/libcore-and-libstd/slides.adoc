# libcore and libstd
[Table of Contents](toc/english.html)

---

Rust ships with two base libraries, libcore and libstd.

---

libcore contains all types necessary to make the language work.

libcore is allocation free.

---

libcore is usually not used directly, as its public interface is reexported by libstd.

---

libstd contains standard functions, for example to interact with the file system. It has a dependency to the platform libc.

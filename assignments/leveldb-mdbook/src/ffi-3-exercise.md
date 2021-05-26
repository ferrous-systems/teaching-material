
# Exercise: Iterate over database contents

In this last part we'll create an `Iterator` for looping over everything stored in our database.

## Preparation

The iterator functionality is exposed by the sys crate as follows:

* `leveldb_create_iterator`: Creates an opaque `leveldb_iterator_t` handle
* `leveldb_iter_seek_to_first`: Starts the iteration by seeking to the first item
* `leveldb_iter_next`: Advances iteration by one element
* `leveldb_iter_value`: Reads the value at the current iterator position
* `leveldb_iter_valid`: Indicates whether the iterator is currently valid


An iterator is position invalid before seeking to the first item and after it has advanced beyond the last one. Reading its value returns *non-owned* data.

## Your tasks

✅ Implement an iterator handle. It should fully encapsulate any `unsafe` code.

✅ Implement an `Iterator` type that holds the necessary state and makes use of the handle.

✅ Implement `pub fn iter(&self) -> Iterator` for your `Database` struct.

✅ Implement [`std::iter::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) for your `Iterator` type. Its items should be of type `Box<[u8]>`.

✅ Write a test case to verify that your iterator returns all items lexicographically sorted by key. Also test with an empty database.

✅ Make the `Iterator` type reference the Database that created it. What has changed and what are the benefits?

✅ Bonus task: what could be used instead of the `Database` reference that achieves the same goal but consumes no memory?

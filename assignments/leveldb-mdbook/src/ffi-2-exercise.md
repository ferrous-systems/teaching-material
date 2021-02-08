# Exercise: Reading and writing database contents

Now that you have an open database, it's time to interact with it by storing and retrieving data. 

## Preparation

You'll need a few more items from the sys crate:
* `leveldb_readoptions_t`: opaque type to specify read operation options
* `leveldb_writeoptions_t`: opaque type to specify write operation options
* `leveldb_readoptions_create`: creates a default `readoptions_t`
* `leveldb_readoptions_destroy`: deallocates `readoptions_t`
* `leveldb_writeoptions_create`: creates a default `writeoptions_t`
* `leveldb_writeoptions_destroy`: deallocates `writeoptions_t`
* `leveldb_put`: writes a binary value for a given binary key
* `leveldb_get`: reads a binary value for a given binary key. Returns a `null` pointer for "not found", an *owned* object otherwise.
* `leveldb_free`: deallocates a value object returned by `leveldb_get`

## Your tasks

✅ Implement two functions on your `Database` type: 
- `pub fn put(&self, key: &[u8], data: &[u8]) -> Result<(), Error>`
- `pub fn get(&self, key: &[u8]) -> Result<Option<Box<[u8]>>, Error>`

Be mindful of the API's ownership contract.

✅ Test your implementation.


## Help and hints
- You only need to create (and destroy!) the read/write options objects, not configure them further in any way.
- Stuck with the wrong primitive type? Try casting it.
- `b"a string"` is literal syntax for creating an `&[u8]` slice.
- Here's how to put a slice on the heap (AKA box it):
  ``` 
  let slice = std::slice::from_raw_parts(data as *mut u8, len);
  let result = Box::from(slice);
  ```
  ❗ note that you've now created a copy of the data and still own (and therefore have to free) the raw pointer.
- don't free `null` pointers.

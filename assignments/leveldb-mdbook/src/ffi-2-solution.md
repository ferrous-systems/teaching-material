# Solution for exercise 2: get & put

``` rust
// ... previous code ...

pub struct WriteOptions {
    ptr: NonNull<leveldb_writeoptions_t>,
}

impl WriteOptions {
    pub fn new() -> WriteOptions {
        unsafe {
            let ptr = leveldb_writeoptions_create();
            WriteOptions {
                ptr: NonNull::new_unchecked(ptr),
            }
        }
    }
}

impl Drop for WriteOptions {
    fn drop(&mut self) {
        unsafe { leveldb_writeoptions_destroy(self.ptr.as_ptr()) }
    }
}

pub struct ReadOptions {
    ptr: NonNull<leveldb_readoptions_t>,
}

impl ReadOptions {
    pub fn new() -> ReadOptions {
        unsafe {
            let ptr = leveldb_readoptions_create();
            ReadOptions {
                ptr: NonNull::new_unchecked(ptr),
            }
        }
    }
}

impl Drop for ReadOptions {
    fn drop(&mut self) {
        unsafe { leveldb_readoptions_destroy(self.ptr.as_ptr()) }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    // ... previous code ...
    GetFail,
    PutFail,
}

impl Database {
    // ... previous code ...

    pub fn get(&self, key: &[u8]) -> Result<Option<Box<[u8]>>, Error> {
        unsafe {
            let read_options = ReadOptions::new();
            let mut len: size_t = 0;
            let mut error = ptr::null_mut();

            let data = leveldb_get(
                self.handle.ptr.as_ptr(),
                read_options.ptr.as_ptr(),
                key.as_ptr() as *const i8,
                key.len(),
                &mut len,
                &mut error,
            );

            if error == ptr::null_mut() {
                if data == ptr::null_mut() {
                    Ok(None)
                } else {
                    let slice = std::slice::from_raw_parts(data as *mut u8, len);

                    let result = Box::from(slice);

                    leveldb_free(data as *mut c_void);

                    Ok(Some(result))
                }
            } else {
                leveldb_free(*error as *mut c_void);
                Err(Error::GetFail)
            }
        }
    }

    pub fn put(&self, key: &[u8], data: &[u8]) -> Result<(), Error> {
        unsafe {
            let write_options = WriteOptions::new();
            let mut error = ptr::null_mut();

            leveldb_put(
                self.handle.ptr.as_ptr(),
                write_options.ptr.as_ptr(),
                key.as_ptr() as *const i8,
                key.len(),
                data.as_ptr() as *const i8,
                data.len(),
                &mut error,
            );

            if error == ptr::null_mut() {
                Ok(())
            } else {
                leveldb_free(*error as *mut c_void);
                Err(Error::PutFail)
            }
        }
    }
}

#[cfg(test)]
mod test {
    // ... previous code ...

    #[test]
    fn test_read_write() {
        let tmp = TempDir::new("test_read_write").unwrap();

        let mut options = Options::new();
        options.create_if_missing(true);

        let database = Database::open(tmp.path().join("database"), options).unwrap();

        let key: &[u8] = b"test";
        let missing_key: &[u8] = b"test_missing";
        let value: &[u8] = b"test";

        database.put(key, value).unwrap();

        let result = database.get(key);
        assert_eq!(result, Ok(Some(Box::from(value))));

        let result = database.get(missing_key);
        assert_eq!(result, Ok(None));
    }
}

```

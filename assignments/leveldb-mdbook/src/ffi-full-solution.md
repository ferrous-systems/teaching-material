# Full solution
```rust
use libc::{c_void, size_t};
use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr;
use std::ptr::NonNull;

use leveldb_sys::{
    leveldb_close, leveldb_create_iterator, leveldb_free, leveldb_get, leveldb_iter_destroy,
    leveldb_iter_next, leveldb_iter_seek_to_first, leveldb_iter_valid, leveldb_iter_value,
    leveldb_iterator_t, leveldb_open, leveldb_options_create, leveldb_options_destroy,
    leveldb_options_set_create_if_missing, leveldb_options_t, leveldb_put,
    leveldb_readoptions_create, leveldb_readoptions_destroy, leveldb_readoptions_t, leveldb_t,
    leveldb_writeoptions_create, leveldb_writeoptions_destroy, leveldb_writeoptions_t,
};

struct DBHandle {
    ptr: NonNull<leveldb_t>,
}

impl Drop for DBHandle {
    fn drop(&mut self) {
        unsafe { leveldb_close(self.ptr.as_ptr()) }
    }
}

pub struct Options {
    ptr: NonNull<leveldb_options_t>,
}

impl Drop for Options {
    fn drop(&mut self) {
        unsafe { leveldb_options_destroy(self.ptr.as_ptr()) }
    }
}

impl Options {
    pub fn new() -> Options {
        unsafe {
            let ptr = leveldb_options_create();
            Options {
                ptr: NonNull::new_unchecked(ptr),
            }
        }
    }

    pub fn create_if_missing(&mut self, value: bool) {
        unsafe { leveldb_options_set_create_if_missing(self.as_ptr(), value as u8) }
    }

    fn as_ptr(&self) -> *mut leveldb_options_t {
        self.ptr.as_ptr()
    }
}

pub struct Database {
    handle: DBHandle,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    OpenFail(String),
    // exercise 2 (get/put)
    GetFail(String),
    // exercise 2 (get/put)
    PutFail(String),
    InvalidString,
}

unsafe fn into_rust_string(ptr: *const i8) -> String {
    let error_s = CStr::from_ptr(ptr).to_string_lossy().to_string();
    leveldb_free(ptr as *mut c_void);
    error_s
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P, options: Options) -> Result<Database, Error> {
        let mut error = ptr::null_mut();

        let c_string = CString::new(path.as_ref().to_str().ok_or(Error::InvalidString)?)
            .map_err(|_| Error::InvalidString)?;
        unsafe {
            let db = leveldb_open(options.as_ptr(), c_string.as_ptr(), &mut error);

            if error == ptr::null_mut() {
                Ok(Database {
                    handle: DBHandle {
                        ptr: NonNull::new_unchecked(db),
                    },
                })
            } else {
                Err(Error::OpenFail(into_rust_string(error)))
            }
        }
    }

    // exercise 2 (get/put)
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
                Err(Error::GetFail(into_rust_string(error)))
            }
        }
    }

    // exercise 2 (get/put)
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
                Err(Error::PutFail(into_rust_string(error)))
            }
        }
    }

    // exercise 3 (Iterator)
    pub fn iter(&self) -> Iterator<'_> {
        let read_options = ReadOptions::new();

        let handle = IteratorHandle::new(self, read_options);

        Iterator {
            handle: handle,
            start: true,
            database: self,
        }
    }
}

// exercise 3 (Iterator)
pub struct Iterator<'iterator> {
    handle: IteratorHandle,
    start: bool,
    #[allow(unused)]
    database: &'iterator Database,
}

// exercise 3 (Iterator)
impl<'iterator> Iterator<'iterator> {
    fn read_current(&self) -> Option<Box<[u8]>> {
        unsafe {
            if !self.handle.valid() {
                return None;
            };

            let data = self.handle.value();

            let slice = std::slice::from_raw_parts(data.0 as *mut u8, data.1);

            Some(Box::from(slice))
        }
    }
}

// exercise 2 (get/put)
pub struct WriteOptions {
    ptr: NonNull<leveldb_writeoptions_t>,
}

// exercise 2 (get/put)
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

// exercise 2 (get/put)
impl Drop for WriteOptions {
    fn drop(&mut self) {
        unsafe { leveldb_writeoptions_destroy(self.ptr.as_ptr()) }
    }
}

// exercise 2 (get/put)
pub struct ReadOptions {
    ptr: NonNull<leveldb_readoptions_t>,
}

// exercise 2 (get/put)
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

// exercise 2 (get/put)
impl Drop for ReadOptions {
    fn drop(&mut self) {
        unsafe { leveldb_readoptions_destroy(self.ptr.as_ptr()) }
    }
}
// exercise 3 (Iterator)
struct IteratorHandle {
    ptr: NonNull<leveldb_iterator_t>,
}

// exercise 3 (Iterator)
impl IteratorHandle {
    fn new(database: &Database, read_options: ReadOptions) -> IteratorHandle {
        unsafe {
            let iterator_ptr =
                leveldb_create_iterator(database.handle.ptr.as_ptr(), read_options.ptr.as_ptr());

            leveldb_iter_seek_to_first(iterator_ptr);

            IteratorHandle {
                ptr: NonNull::new_unchecked(iterator_ptr),
            }
        }
    }

    fn next(&self) {
        unsafe { leveldb_iter_next(self.ptr.as_ptr()) };
    }

    fn valid(&self) -> bool {
        unsafe { leveldb_iter_valid(self.ptr.as_ptr()) != 0 }
    }

    fn value(&self) -> (*const i8, usize) {
        unsafe {
            let mut len = 0;

            let data = leveldb_iter_value(self.ptr.as_ptr(), &mut len);

            (data, len)
        }
    }
}

// exercise 3 (Iterator)
impl Drop for IteratorHandle {
    fn drop(&mut self) {
        unsafe { leveldb_iter_destroy(self.ptr.as_ptr()) }
    }
}

// exercise 3 (Iterator)
impl<'iterator> std::iter::Iterator for Iterator<'iterator> {
    type Item = Box<[u8]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start {
            self.start = false;

            self.read_current()
        } else {
            self.handle.next();

            self.read_current()
        }
    }
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_open() {
        let tmp = TempDir::new("test_open").unwrap();

        let mut options = Options::new();
        options.create_if_missing(true);

        let database = Database::open(tmp.path().join("database"), options);
        assert!(database.is_ok());
    }

    #[test]
    fn test_create_open_fails() {
        let mut options = Options::new();
        options.create_if_missing(true);

        let database = Database::open("/invalid/location", options);
        match database {
            Err(Error::OpenFail(_)) => {}
            _ => panic!(),
        }
    }

    #[test]
    fn test_open_nonexistent_fails() {
        let options = Options::new();

        let database = Database::open("/invalid/location", options);
        match database {
            Err(Error::OpenFail(_)) => {}
            _ => panic!(),
        }
    }

    // exercise 2 (get/put)
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

    // exercise 3 (Iterator)
    #[test]
    fn test_iter() {
        let tmp = TempDir::new("test_iter").unwrap();

        let mut options = Options::new();
        options.create_if_missing(true);

        let database = Database::open(tmp.path().join("database"), options).unwrap();

        let key1: &[u8] = b"test1";
        let key2: &[u8] = b"test2";
        let key3: &[u8] = b"test3";

        let value1: &[u8] = b"value1";
        let value2: &[u8] = b"value2";
        let value3: &[u8] = b"value3";

        database.put(key1, value1).unwrap();
        database.put(key2, value2).unwrap();
        database.put(key3, value3).unwrap();

        let mut iter = database.iter();

        assert_eq!(iter.next(), Some(Box::from(value1)));
        assert_eq!(iter.next(), Some(Box::from(value2)));
        assert_eq!(iter.next(), Some(Box::from(value3)));
        assert_eq!(iter.next(), None);
    }
}

```
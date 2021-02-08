# Solution for exercise 1: open & close

``` rust
use libc::{c_void, size_t};
use std::ffi::CString;
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
    OpenFail,
    NonUtf8Path,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P, options: Options) -> Result<Database, Error> {
        let mut error = ptr::null_mut();

        let c_string = CString::new(path.as_ref().to_str().ok_or(Error::NonUtf8Path)?)
            .map_err(|_| Error::NonUtf8Path)?;
        unsafe {
            let db = leveldb_open(options.as_ptr(), c_string.as_ptr(), &mut error);

            if error == ptr::null_mut() {
                Ok(Database {
                    handle: DBHandle {
                        ptr: NonNull::new_unchecked(db),
                    },
                })
            } else {
                Err(Error::OpenFail)
            }
        }
    }
}

#[cfg(test)]
mod test {
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
}
```
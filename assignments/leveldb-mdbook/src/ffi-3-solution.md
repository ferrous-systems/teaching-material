# Solution for exercise 3: Iterators

``` rust
// ... previous code ...

struct IteratorHandle {
    ptr: NonNull<leveldb_iterator_t>,
}

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

impl Drop for IteratorHandle {
    fn drop(&mut self) {
        unsafe { leveldb_iter_destroy(self.ptr.as_ptr()) }
    }
}

impl Database {
    // ... previous code ...

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

pub struct Iterator<'iterator> {
    handle: IteratorHandle,
    start: bool,
    #[allow(unused)]
    database: &'iterator Database,
}

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
    // ... previous code ...

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

## Bonus task solution

Lifetime relationships can be expressed without occupying space in application memory by using [`PhantomData<T>`](https://doc.rust-lang.org/std/marker/struct.PhantomData.html):

``` rust
use std::marker::PhantomData;

pub struct Iterator<'iterator> {
    handle: IteratorHandle,
    start: bool,
    phantom: PhantomData<&'iterator Database>,
}

// usage:
Iterator {
    handle: handle,
    start: true,
    phantom: PhantomData,
}
```
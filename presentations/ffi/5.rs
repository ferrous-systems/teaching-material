extern "C" {
    pub fn leveldb_open(options: *const leveldb_options_t, name: *const c_char, errptr: *mut *mut c_char) -> *mut leveldb_t;
    pub fn leveldb_close(db: *mut leveldb_t);
    //....
}
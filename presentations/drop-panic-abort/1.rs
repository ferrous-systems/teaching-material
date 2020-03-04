struct LevelDB {
    handle: *mut leveldb_database_t
}

impl Drop for LevelDB {
    fn drop(&mut self) {
        unsafe { leveldb_close(self.handle) };
    }
}

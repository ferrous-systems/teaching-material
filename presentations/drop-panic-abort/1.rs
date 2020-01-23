struct LevelDB {
    handle: *leveldb_database_t
}

impl Drop for LevelDB {
    fn drop(&self) {
        unsafe { leveldb_close(self.handle) };
    }
}
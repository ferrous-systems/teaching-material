struct Thing(*mut String);

unsafe impl Send for Thing {}
unsafe impl Sync for Thing {}
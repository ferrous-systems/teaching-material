unsafe {
    (*weechat_plugin).hook_command.unwrap()(
        plugin,
        "connector\0".as_ptr() as *const i8,
        "Building pipes between processes\0".as_ptr() as *const i8,
        "a | b\0".as_ptr() as *const i8,
        "a: Source pipe \nb: Sink pipe\0".as_ptr() as *const i8,
        ::std::ptr::null_mut(),
        Some(&*connector_callback),
        ::std::ptr::null_mut(),
        ::std::ptr::null_mut(),
    );
}

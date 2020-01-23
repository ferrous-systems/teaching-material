#[no_mangle]
pub extern "C" fn weechat_plugin_init(
    plugin: *mut t_weechat_plugin,
    argc: c_int,
    argv: *const *const c_char,
) -> c_int {
    unsafe { weechat_plugin = plugin };

    // ...

    WEECHAT_RC_OK as i32
}

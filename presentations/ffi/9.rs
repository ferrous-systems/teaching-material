unsafe {
    let mut severity = 0;
    let description: *mut c_char = 
        bindings::MagickGetException(self.0, &mut severity);

    let string = CStr::from_ptr(description)
                        .to_string_lossy()
                        .into_owned();

    bindings::MagickRelinquishMemory(description as *mut _);

    string
}

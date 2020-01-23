struct MagickWandInstance(*mut bindings::MagickWand);

impl MagickWandInstance {
    fn new() -> Self {
        MagickWandInstance(unsafe { bindings::NewMagickWand() })
    }
}

impl Drop for MagickWandInstance {
    fn drop(&mut self) {
        unsafe { bindings::MagickWandTerminus() };
    }
}

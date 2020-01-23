// ... finding imagemagick via pkgconf ...

let bindings = {
    let mut builder = bindgen::Builder::default()
        .header_contents("bindings.h", &header)
        .whitelist_function(".*Magick.*")
        .whitelist_var("MAGICK.*");

    for path in &magick_wand.include_paths {
        builder = builder.clang_arg("-isystem")
                    .clang_arg(path.to_string_lossy());
    }
    builder.generate().expect("Couldn't generate bindings")
};

bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
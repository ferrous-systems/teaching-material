use cc;

fn main() {
	cc::Build::new()
		.file("src/cool_library.c")
		.compile("cool_library");
}

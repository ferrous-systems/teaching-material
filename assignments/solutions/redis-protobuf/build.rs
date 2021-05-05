fn main() -> std::io::Result<()> {
    let protos = ["src/phonebook.proto", "src/config.proto"];
    for proto in &protos {
        println!("cargo:rerun-if-changed={}", proto);
    }
    prost_build::compile_protos(&protos, &["src/"])?;
    Ok(())
}

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("proto_descriptor.bin");

    tonic_prost_build::configure()
        .build_server(false)
        .file_descriptor_set_path(&descriptor_path)
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", "::pbjson_types")
        .compile_protos( // Note: .compile() is often aliased to .compile_protos() in newer versions
                         &["proto/a2a.proto"],
                         &["proto"],
        )?;
    
    let descriptor_bytes = std::fs::read(&descriptor_path)?;

    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_bytes)?
        .build(&[".a2a"])?;

    Ok(())
}

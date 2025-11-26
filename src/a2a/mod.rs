// Include the generated code
tonic::include_proto!("a2a.v1");

// Include pbjson generated impls
// Note: pbjson generates a file named `a2a.v1.serde.rs` usually, but we need to check.
// Actually, pbjson generates `serde` module inside the package?
// Let's try to include it.
// pbjson-build usually generates a file that we need to include.
// If we use `pbjson_build::Builder::new().build(&[".a2a"])`, it generates `a2a.v1.serde.rs`?
// No, it generates `a2a.v1.rs`? No, that's tonic.
// It generates `a2a.v1.serde.rs` in OUT_DIR.
// We need to include it.
// But `tonic::include_proto` only includes the main file.
// We might need to manually include the serde file.
// Or maybe pbjson modifies the existing file? No.

// Let's assume it generates `a2a.v1.serde.rs` and we need to include it.
// But since `AgentCard` is in `self`, the impls need to be here too.
include!(concat!(env!("OUT_DIR"), "/a2a.v1.serde.rs"));

mod tests;


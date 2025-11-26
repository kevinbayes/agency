# A2A Implementation Plan

## Goal
Implement the A2A (Agent-to-Agent) protocol in Rust by generating the models directly from the `a2a.proto` specification.
This approach ensures strict adherence to the protocol and reduces manual maintenance.

## References
- Proto Specification: `a2a.proto` (v1)
- Java Implementation: `AgentCard.java` (for reference on behavior, but not for model structure)

## Proposed Changes

### Dependencies
The project already has `tonic` and `prost` in `[dependencies]`.
I will need to add `tonic-build` to `[build-dependencies]` in `Cargo.toml`.

### Proto File
1.  Create a directory `proto/` in the project root.
2.  Download `a2a.proto` into `proto/a2a.proto`.

### Build Script
1.  Create `build.rs` in the project root (if it doesn't exist) or update it.
2.  Configure `tonic_build` to compile `proto/a2a.proto`.

### Module Structure
1.  Create `src/a2a/mod.rs`.
2.  Include the generated code using `tonic::include_proto!("a2a.v1")`.
3.  Re-export the necessary structs (`AgentCard`, `AgentSkill`, etc.) for easier access.

## Verification Plan

### Automated Tests
- **Compilation Check**: `cargo build` to ensure the build script runs and generates valid Rust code.
- **Unit Tests**: Create `src/a2a/tests.rs` to verify:
    - Instantiation of generated structs.
    - Serialization/Deserialization (if `serde` support is enabled in `tonic-build` config, otherwise we might need to add `#[derive(Serialize, Deserialize)]` via `tonic-build` attributes).
    - `cargo test`

### Manual Verification
- Inspect the generated code (usually in `target/debug/build/...`) to confirm it matches expectations.

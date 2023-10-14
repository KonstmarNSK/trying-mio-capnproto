fn main() {
    // RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build --release -Z build-std=panic_abort,std --target x86_64-unknown-linux-musl
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/point.capnp")
        .output_path("src/generated/capnp")
        // .file("schema/bar.capnp")
        .run().expect("schema compiler command");
}
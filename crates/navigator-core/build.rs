use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "../../proto/navigator.proto";
    let out_dir = PathBuf::from("src/proto");

    // Ensure the output directory exists
    std::fs::create_dir_all(&out_dir)?;

    // Configure tonic-build
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(&out_dir)
        .compile_protos(&[proto_file], &["../../proto"])?;

    // Tell cargo to rerun if the proto file changes
    println!("cargo:rerun-if-changed={proto_file}");

    Ok(())
}

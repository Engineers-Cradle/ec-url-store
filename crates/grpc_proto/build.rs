fn main() {
    let out_dir = std::path::PathBuf::from("src/gen");
    
    if !out_dir.exists() {
        std::fs::create_dir_all(&out_dir).unwrap();
    }

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("proto_descriptor.bin"))
        .out_dir(out_dir)
        .compile_protos(&["proto/req.proto"], &["proto"])
        .unwrap();
}
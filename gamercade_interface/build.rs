fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/chat.proto").unwrap();
    // tonic_build::compile_protos("proto/common.proto").unwrap();
    // tonic_build::compile_protos("proto/games.proto").unwrap();
    // tonic_build::compile_protos("proto/images.proto").unwrap();
    // tonic_build::compile_protos("proto/platform.proto").unwrap();
    // tonic_build::compile_protos("proto/users.proto").unwrap();

    tonic_build::configure()
        .out_dir("src/output")
        .include_file("mod.rs")
        .compile(
            &[
                "proto/chat.proto",
                "proto/common.proto",
                "proto/games.proto",
                "proto/images.proto",
                "proto/platform.proto",
                "proto/users.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}

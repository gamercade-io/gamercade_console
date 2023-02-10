fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/chat.proto").unwrap();
    tonic_build::compile_protos("proto/common.proto").unwrap();
    tonic_build::compile_protos("proto/games.proto").unwrap();
    tonic_build::compile_protos("proto/images.proto").unwrap();
    tonic_build::compile_protos("proto/platform.proto").unwrap();
    tonic_build::compile_protos("proto/users.proto").unwrap();

    Ok(())
}

use std::fs;

const OUTPUT_DIR: &str = "src/output";
const PROTO_DIR: &str = "proto";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare the output directory
    if fs::read_dir(OUTPUT_DIR).is_ok() {
        fs::remove_dir_all(OUTPUT_DIR)?;
    };
    fs::create_dir(OUTPUT_DIR)?;

    // Gather the list of found proto files
    let protos = fs::read_dir(PROTO_DIR)
        .expect("Failed to find proto directory.")
        .flat_map(|file| {
            file.map(|file| match file.path().extension() {
                Some(extension) if extension == "proto" => {
                    Some(file.path().to_string_lossy().to_string())
                }
                _ => None,
            })
        })
        .flatten()
        .collect::<Vec<String>>();

    // Compile and output the proto files
    tonic_build::configure()
        .out_dir(OUTPUT_DIR)
        .protoc_arg("--experimental_allow_proto3_optional")
        .include_file("mod.rs")
        .compile(&protos, &[PROTO_DIR])?;

    Ok(())
}

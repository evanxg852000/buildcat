use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
     println!("cargo:rerun-if-changed=protobuf/*");
    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");

    let input_files = [
        "protobuf/build/bazel/remote/asset/v1/remote_asset.proto",
        // "protobuf/remote_execution_v2.proto",
        // "protobuf/remote_logstream_v1.proto",
    ];
    let output_dir = PathBuf::from("src/protobuf");

    tonic_build::configure()
        .btree_map(["."])
        .type_attribute(
            "services.Node",
            "#[derive(Eq, serde::Serialize, serde::Deserialize)]",
        )
        .out_dir(&output_dir)
        .file_descriptor_set_path(output_dir.join("services_descriptor.bin"))
        .include_file("mod.rs")
        .compile_protos_with_config(config, &input_files, &["protobuf"])?;
    Ok(())
}

use protoc_bin_vendored::protoc_bin_path;
use std::{
    fs::{self},
    path::PathBuf,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
     slint_build::compile("ui/app-window.slint").expect("Slint build failed");
    let protoc = protoc_bin_path().unwrap();
    unsafe { std::env::set_var("PROTOC", protoc) };
    let out_dir = PathBuf::from("./src/generated");
    fs::create_dir_all(&out_dir)?;
    tonic_build::configure()
        .out_dir(&out_dir)
        .message_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        //.file_descriptor_set_path(out_dir.join("reflection.bin"))
        .build_server(false)
        .build_client(true)
        .compile_protos(&["open-pi-scope.proto"], &["proto"])?;
    Ok(())
} 
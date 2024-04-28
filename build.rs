use std::env;
use std::path::{PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("db_api_descriptor.bin"))
        .compile(&["proto/db_api.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/db_api.proto")?;

    Ok(())
}


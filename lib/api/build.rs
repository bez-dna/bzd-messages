use std::{
    env::{self},
    path::PathBuf,
};

use bzd_lib::error::Error;

fn main() -> Result<(), Error> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    tonic_prost_build::configure()
        .file_descriptor_set_path(out_dir.join("messages_descriptor.bin"))
        .compile_protos(&["src/messages.proto"], &["src"])?;

    tonic_prost_build::configure()
        .file_descriptor_set_path(out_dir.join("topics_descriptor.bin"))
        .compile_protos(&["src/topics.proto"], &["src"])?;

    tonic_prost_build::configure()
        .build_server(false)
        .build_client(false)
        .file_descriptor_set_path(out_dir.join("events_descriptor.bin"))
        .compile_protos(&["src/events.proto"], &["src"])?;

    Ok(())
}

use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let user_proto = "/Users/yaojun/rust/rustx/myproject/proto/user/user.proto";
    let proto_path = "/Users/yaojun/rust/rustx/myproject/proto/";

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(&out_dir)
        .file_descriptor_set_path(&out_dir.join("user_descriptor.bin"))
        .compile(&[user_proto], &[proto_path])
        .unwrap_or_else(|err| panic!("protobuf compile failed: {}", err));
}
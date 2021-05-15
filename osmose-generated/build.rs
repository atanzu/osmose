extern crate protobuf_codegen_pure;

use std::fs;
use std::path::Path;
use protobuf_codegen_pure::Customize;

fn main() {
    let generated_dir = "./src/generated";

    if Path::new(&generated_dir).exists() {
        fs::remove_dir_all(&generated_dir).unwrap();
    }
    fs::create_dir(&generated_dir).unwrap();

    protobuf_codegen_pure::Codegen::new()
        .customize(Customize {
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .out_dir(generated_dir)
        .input("protos/osmose.proto")
        .include("protos")
        .run()
        .unwrap();
}

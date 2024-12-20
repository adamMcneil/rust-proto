// use std::io::Result;
// fn main() -> Result<()> {
//     println!("I am running");
//     println!("I am running");
//     println!("I am running");
//     println!("I am running");
//     println!("I am running");
//     prost_build::compile_protos(&["src/items.proto"], &["src/"])?;
//     Ok(())
// }
// build.rs

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    fs::write(
        &dest_path,
        "pub fn message() -> &'static str {
            \"Hello, World!\"
        }
        ",
    )
    .unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}

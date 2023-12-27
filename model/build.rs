use std::{process, path::PathBuf};

fn main() {
    let data_build_script = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .join("data").join("bin").join("build.py");
        
    process::Command::new(data_build_script)
        .output()
        .expect("Data build script failed");

    println!("cargo:rerun-if-changed=../data");
}
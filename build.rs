use std::{
    env::{current_dir, var},
    error::Error,
    fs::{copy, create_dir_all, read_dir},
    path::PathBuf,
};

use cc::Build;

fn compile_liblz4() -> Result<(), Box<dyn Error>> {
    let mut compiler = Build::new();

    compiler
        .define("XXH_NAMESPACE", "LZ4_")
        .file("liblz4/lib/lz4.c")
        .file("liblz4/lib/lz4frame.c")
        .file("liblz4/lib/lz4hc.c")
        .file("liblz4/lib/xxhash.c")
        .opt_level(3);

    let target = var("TARGET")?;
    if target == "i686-pc-windows-gnu" {
        compiler.flag("-fno-tree-vectorize");
    }

    compiler.compile("liblz4.a");

    let src_dir = current_dir()?.join("liblz4").join("lib");
    let out_dir = PathBuf::from(var("OUT_DIR")?);
    let inc_dir = out_dir.join("include");

    create_dir_all(&inc_dir)?;
    for entry in read_dir(&src_dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if file_name.ends_with(".h") {
            copy(&entry.path(), &inc_dir.join(entry.file_name()))?;
        }
    }
    println!("cargo:root={}", out_dir.display());

    Ok(())
}

fn main() { compile_liblz4().expect("error compiling liblz4"); }

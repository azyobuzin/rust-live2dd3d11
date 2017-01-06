extern crate gcc;

use std::env;

fn main() {
    // https://github.com/alexcrichton/gcc-rs/blob/b1601a6bc2c35169cb566aafeac956b9cec0226c/src/lib.rs#L468-L473
    let crt_static = env::var("CARGO_CFG_TARGET_FEATURE").ok()
        .map_or(false, |x| x.contains("crt-static"));

    let mut config = gcc::Config::new();

    if let Some(include_dir) = env::var_os("LIVE2DD3D11_INCLUDE_DIR") {
        config.include(include_dir);
    }

    let mut lib_path = std::path::PathBuf::new();

    if let Some(lib_dir) = env::var_os("LIVE2DD3D11_LIB_DIR") {
        lib_path.push(lib_dir);
    }

    lib_path.push(if crt_static { "live2d_directX_mtd.lib" } else { "live2d_directX_mdd.lib" });

    config.cpp(true)
        .object(lib_path)
        .define("_DEBUG", None)
        .file("src/wrapper.cpp")
        .compile("libl2dwrapper.a");
}

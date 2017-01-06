extern crate gcc;

use std::env;

fn main() {
    // https://github.com/alexcrichton/gcc-rs/blob/b1601a6bc2c35169cb566aafeac956b9cec0226c/src/lib.rs#L902
    let is_debug = env::var_os("PROFILE")
        .map_or(false, |x| &x == "debug");

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

    lib_path.push(match (is_debug, crt_static) {
        (false, false) => "live2d_directX_md.lib",
        (true, false) => "live2d_directX_mdd.lib",
        (false, true) => "live2d_directX_mt.lib",
        (true, true) => "live2d_directX_mtd.lib",
    });

    if is_debug {
        config.define("_DEBUG", None);
    }

    config.cpp(true)
        .object(lib_path)
        .file("src/wrapper.cpp")
        .compile("libl2dwrapper.a");
}

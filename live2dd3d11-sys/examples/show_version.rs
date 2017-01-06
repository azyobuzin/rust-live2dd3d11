extern crate live2dd3d11_sys;

use live2dd3d11_sys::*;

fn main() {
    println!(
        "{} ({})",
         getVersionStr().to_str().unwrap(),
         getVersionNo()
    );
}

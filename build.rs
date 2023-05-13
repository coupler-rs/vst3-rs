mod gen;

use std::env;

use gen::generate;

fn main() {
    println!("cargo:rerun-if-env-changed=VST3_SDK_DIR");
    let vst3_sdk_dir = env::var("VST3_SDK_DIR").expect("please provide a value for VST3_SDK_DIR");

    let out_dir = env::var("OUT_DIR").unwrap();

    generate(&vst3_sdk_dir, &out_dir).unwrap();
}

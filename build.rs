mod gen;

use std::env;
use std::process;

use gen::generate;

fn main() {
    println!("cargo:rerun-if-env-changed=VST3_SDK_DIR");
    let vst3_sdk_dir = if let Ok(dir) = env::var("VST3_SDK_DIR") {
        dir
    } else {
        eprintln!("please provide a value for VST3_SDK_DIR");
        process::exit(1);
    };

    let out_dir = env::var("OUT_DIR").unwrap();

    if let Err(err) = generate(&vst3_sdk_dir, &out_dir) {
        eprintln!("{}", err);
        process::exit(1);
    }
}

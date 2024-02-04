use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::process;

use vst3_bindgen::generate;

fn main() {
    println!("cargo:rerun-if-env-changed=VST3_SDK_DIR");
    let vst3_sdk_dir = if let Ok(dir) = env::var("VST3_SDK_DIR") {
        dir
    } else {
        eprintln!("please provide a value for VST3_SDK_DIR");
        process::exit(1);
    };

    println!("cargo:rerun-if-changed={}", vst3_sdk_dir);

    let out_dir = env::var("OUT_DIR").unwrap();

    let bindings = File::create(Path::new(&out_dir).join("bindings.rs")).unwrap();
    let sink = BufWriter::new(bindings);

    if let Err(err) = generate(Path::new(&vst3_sdk_dir), None, sink) {
        eprintln!("{}", err);
        process::exit(1);
    }
}

use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let out_path = env::var("OUT_DIR").unwrap();
    let _bindings = File::create(Path::new(&out_path).join("bindings.rs")).unwrap();
}

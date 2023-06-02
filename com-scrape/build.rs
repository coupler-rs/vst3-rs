use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut dst = File::create(Path::new(&out_dir).join("host-target.txt")).unwrap();
    let host_target = env::var("TARGET").unwrap();
    dst.write_all(host_target.as_bytes()).unwrap();
}

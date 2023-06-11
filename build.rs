use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::process;
use std::{fs, io};

fn find_headers<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>, io::Error> {
    fn find_headers_inner<P: AsRef<Path>>(dir: P, headers: &mut Vec<PathBuf>) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let metadata = entry.metadata()?;

            let path = entry.path();
            if metadata.file_type().is_dir() {
                find_headers_inner(path, headers)?;
            } else {
                if path.extension().map(|ext| ext == "h").unwrap_or(false) {
                    headers.push(path);
                }
            }
        }

        Ok(())
    }

    let mut headers = Vec::new();
    find_headers_inner(dir, &mut headers)?;

    Ok(headers)
}

fn parse_iid(tokens: &[String]) -> Option<String> {
    if let Some(first) = tokens.first() {
        if first == "DECLARE_CLASS_IID" {
            return Some(format!(
                "pub const {}_iid: TUID = uid({}, {}, {}, {});",
                tokens[2], tokens[4], tokens[6], tokens[8], tokens[10]
            ));
        }
    }

    None
}

fn generate(sdk_dir: &str) -> Result<(), Box<dyn Error>> {
    let pluginterfaces_path = Path::new(&sdk_dir).join("pluginterfaces");
    let headers = find_headers(&pluginterfaces_path)?;

    let skip_headers = HashSet::from([
        Path::new("pluginterfaces/base/funknownimpl.h"),
        Path::new("pluginterfaces/base/ustring.h"),
        Path::new("pluginterfaces/test/itest.h"),
        Path::new("pluginterfaces/vst/ivsttestplugprovider.h"),
    ]);

    let mut source = String::new();
    for header in &headers {
        let relative = header.strip_prefix(&sdk_dir).unwrap();
        if skip_headers.contains(relative) {
            continue;
        }

        let name = relative.to_str().unwrap();

        use std::fmt::Write;
        writeln!(source, "#include \"{}\"", name)?;
    }

    let out_dir = env::var("OUT_DIR").unwrap();

    let bindings = File::create(Path::new(&out_dir).join("bindings.rs"))?;
    let sink = BufWriter::new(bindings);

    com_scrape::Generator::default()
        .skip_types(&[
            "Adopt",
            "ConstStringTable",
            "FUID",
            "FReleaser",
            "LARGE_INT",
        ])
        .skip_interface_trait("FUnknown")
        .constant_parser(parse_iid)
        .iid_generator(|name| format!("crate::tuid_as_guid({name}_iid)"))
        .query_interface_fn("crate::FUnknown_query_interface")
        .add_ref_fn("crate::FUnknown_add_ref")
        .release_fn("crate::FUnknown_release")
        .include_path(&sdk_dir)
        .generate(source, sink)?;

    Ok(())
}

fn main() {
    println!("cargo:rerun-if-env-changed=VST3_SDK_DIR");
    let vst3_sdk_dir = if let Ok(dir) = env::var("VST3_SDK_DIR") {
        dir
    } else {
        eprintln!("please provide a value for VST3_SDK_DIR");
        process::exit(1);
    };

    println!("cargo:rerun-if-changed={}", vst3_sdk_dir);

    if let Err(err) = generate(&vst3_sdk_dir) {
        eprintln!("{}", err);
        process::exit(1);
    }
}

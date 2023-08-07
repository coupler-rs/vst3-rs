//! A binding generator for the VST 3 API. `vst3-bindgen` can be used to generate Rust bindings for
//! the VST 3 API from the original C++ headers.

use std::collections::HashSet;
use std::error::Error;
use std::io::Write;
use std::path::{Path, PathBuf};
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

/// Generates Rust bindings given a path to the VST 3 SDK.
pub fn generate(sdk_dir: &Path, mut sink: impl Write) -> Result<(), Box<dyn Error>> {
    let pluginterfaces_path = sdk_dir.join("pluginterfaces");
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

    writeln!(sink, "mod __bindings {{")?;
    writeln!(sink)?;

    writeln!(sink, "{}", include_str!("support.rs"))?;

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
        .iid_generator(|name| format!("crate::__bindings::tuid_as_guid({name}_iid)"))
        .query_interface_fn("crate::__bindings::FUnknown_query_interface")
        .add_ref_fn("crate::__bindings::FUnknown_add_ref")
        .release_fn("crate::__bindings::FUnknown_release")
        .include_path(&sdk_dir)
        .generate(source, &mut sink)?;

    writeln!(sink)?;
    writeln!(sink, "}}")?;
    writeln!(sink, "pub use __bindings::*;")?;

    Ok(())
}

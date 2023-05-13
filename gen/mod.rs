mod clang;

use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};

use clang::*;

fn find_headers<P: AsRef<Path>>(dir: P) -> io::Result<Vec<PathBuf>> {
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

pub fn visit(cursor: &Cursor, w: &mut impl Write, skip_list: &HashSet<&str>) {
    if cursor.is_in_system_header() {
        return;
    }

    match cursor.kind() {
        Kind::Namespace => {
            cursor.visit_children(|cursor| {
                visit(cursor, w, skip_list);
            });
        }
        Kind::ClassDecl => {
            if cursor.is_definition() {
                let name = cursor.name();
                let name_str = name.to_str().unwrap();
                if !skip_list.contains(name_str) {
                    writeln!(w, "pub struct {};", name_str).unwrap();
                }
            }
        }
        _ => {}
    }
}

pub fn generate(sdk_dir: &str, out_dir: &str) -> Result<(), Box<dyn Error>> {
    let pluginterfaces_path = Path::new(&sdk_dir).join("pluginterfaces");
    let headers = find_headers(&pluginterfaces_path).expect("error scanning directory");

    let mut source = String::new();
    for header in &headers {
        let name = header.strip_prefix(&sdk_dir).unwrap().to_str().unwrap();

        use std::fmt::Write;
        writeln!(source, "#include \"{}\"", name).unwrap();
    }

    let unit = TranslationUnit::new(&source, &sdk_dir).unwrap();

    let bindings = File::create(Path::new(&out_dir).join("bindings.rs")).unwrap();
    let mut w = BufWriter::new(bindings);

    let skip_list = HashSet::from(["ConstStringTable", "FUID", "FVariant", "UString"]);

    unit.cursor().visit_children(|cursor| {
        visit(cursor, &mut w, &skip_list);
    });

    Ok(())
}

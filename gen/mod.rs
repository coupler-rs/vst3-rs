mod clang;
mod parse;
mod print;

use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::{fs, io};

use clang::*;
use parse::*;
use print::*;

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

pub fn generate(sdk_dir: &str, out_dir: &str) -> Result<(), Box<dyn Error>> {
    let pluginterfaces_path = Path::new(&sdk_dir).join("pluginterfaces");
    let headers = find_headers(&pluginterfaces_path).expect("error scanning directory");

    let mut source = String::new();
    for header in &headers {
        let name = header.strip_prefix(&sdk_dir).unwrap().to_str().unwrap();

        use std::fmt::Write;
        writeln!(source, "#include \"{}\"", name)?;
    }

    let unit = TranslationUnit::new(&source, &sdk_dir).unwrap();

    let skip_list = &["ConstStringTable", "FUID", "FVariant", "UString"];
    let namespace = Namespace::parse(&unit.cursor(), skip_list);

    let bindings = File::create(Path::new(&out_dir).join("bindings.rs"))?;
    let mut printer = RustPrinter::new(BufWriter::new(bindings));
    printer.print_namespace(&namespace)?;

    Ok(())
}

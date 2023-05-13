use std::collections::HashSet;
use std::env;
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::{fs, io};

use clang_sys::*;

macro_rules! c_str {
    ($str:literal) => {
        concat!($str, "\0").as_ptr() as *const c_char
    };
}

struct TranslationUnit {
    index: CXIndex,
    unit: CXTranslationUnit,
}

impl TranslationUnit {
    fn new(source: &str, include_path: &str) -> Result<TranslationUnit, ()> {
        let path = CString::new(include_path).unwrap();

        unsafe {
            let index = clang_createIndex(0, 0);

            let args = [
                c_str!("-x"),
                c_str!("c++"),
                c_str!("-I"),
                path.as_ptr() as *const c_char,
            ];

            let filename = c_str!("header.h");
            let mut sources = [CXUnsavedFile {
                Filename: filename,
                Contents: source.as_ptr() as *const c_char,
                Length: source.len() as u64,
            }];

            let mut unit = MaybeUninit::uninit();
            let result = clang_parseTranslationUnit2(
                index,
                filename,
                args.as_ptr(),
                args.len() as c_int,
                sources.as_mut_ptr(),
                sources.len() as u32,
                CXTranslationUnit_None,
                unit.as_mut_ptr(),
            );
            let unit = unit.assume_init();

            if result != CXError_Success {
                clang_disposeIndex(index);
                return Err(());
            }

            Ok(TranslationUnit { index, unit })
        }
    }

    fn visit<F>(&self, mut callback: F)
    where
        F: FnMut(&str),
    {
        extern "C" fn visitor(
            cursor: CXCursor,
            _parent: CXCursor,
            client_data: CXClientData,
        ) -> CXChildVisitResult {
            unsafe {
                let data = &mut *(client_data as *mut Data);

                let kind = clang_getCursorKind(cursor);

                let location = clang_getCursorLocation(cursor);
                if clang_Location_isInSystemHeader(location) == 0 {
                    let is_definition =
                        clang_equalCursors(cursor, clang_getCursorDefinition(cursor)) != 0;
                    if kind == CXCursor_ClassDecl && is_definition {
                        let name = clang_getCursorSpelling(cursor);
                        let name_str = clang_getCString(name);

                        (data.callback)(CStr::from_ptr(name_str).to_str().unwrap());

                        clang_disposeString(name);
                    }

                    if kind == CXCursor_Namespace {
                        clang_visitChildren(cursor, visitor, client_data);
                    }
                }

                CXChildVisit_Continue
            }
        }

        struct Data<'c> {
            callback: &'c mut dyn FnMut(&str),
        }
        let mut data = Data {
            callback: &mut callback,
        };

        unsafe {
            let cursor = clang_getTranslationUnitCursor(self.unit);
            clang_visitChildren(cursor, visitor, &mut data as *mut Data as *mut c_void);
        }
    }
}

impl Drop for TranslationUnit {
    fn drop(&mut self) {
        unsafe {
            clang_disposeTranslationUnit(self.unit);
            clang_disposeIndex(self.index);
        }
    }
}

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

fn main() {
    println!("cargo:rerun-if-env-changed=VST3_SDK_PATH");
    let vst3_sdk_path =
        env::var("VST3_SDK_PATH").expect("please provide a value for VST3_SDK_PATH");

    let pluginterfaces_path = Path::new(&vst3_sdk_path).join("pluginterfaces");
    let headers = find_headers(&pluginterfaces_path).expect("error scanning directory");

    let mut source = String::new();
    for header in &headers {
        let name = header
            .strip_prefix(&vst3_sdk_path)
            .unwrap()
            .to_str()
            .unwrap();

        use std::fmt::Write;
        writeln!(source, "#include \"{}\"", name).unwrap();
    }

    let unit = TranslationUnit::new(&source, &vst3_sdk_path).unwrap();

    let out_path = env::var("OUT_DIR").unwrap();
    let bindings = File::create(Path::new(&out_path).join("bindings.rs")).unwrap();
    let mut w = BufWriter::new(bindings);

    let skip_list = HashSet::from(["ConstStringTable", "FUID", "FVariant", "UString"]);

    unit.visit(|name| {
        if skip_list.contains(name) {
            return;
        }

        writeln!(w, "pub struct {};", name).unwrap();
    });
}

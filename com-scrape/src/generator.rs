use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::clang::*;
use crate::parse::*;
use crate::print::*;

const HOST_TARGET: &'static str = include_str!(concat!(env!("OUT_DIR"), "/host-target.txt"));

// Some target triples are different between rustc and clang.
// See https://github.com/rust-lang/rust-bindgen/blob/05ebcace15a8784e5a5b1001a3b755b866fac901/bindgen/lib.rs#L670
fn rust_to_clang_target(rust_target: &str) -> String {
    if rust_target.starts_with("aarch64-apple-") {
        return "arm64-apple-".to_owned() + &rust_target["aarch64-apple-".len()..];
    }

    rust_target.to_owned()
}

pub struct Generator {
    pub(crate) include_paths: Vec<PathBuf>,
    pub(crate) skip_types: HashSet<String>,
    pub(crate) skip_interface_traits: HashSet<String>,
    pub(crate) constant_parser: Option<Box<dyn Fn(&[String]) -> Option<String>>>,
    pub(crate) iid_generator: Option<Box<dyn Fn(&str) -> String>>,
    pub(crate) query_interface_fn: Option<String>,
    pub(crate) add_ref_fn: Option<String>,
    pub(crate) release_fn: Option<String>,
}

impl Default for Generator {
    fn default() -> Generator {
        Generator {
            include_paths: Vec::new(),
            skip_types: HashSet::new(),
            skip_interface_traits: HashSet::new(),
            constant_parser: None,
            iid_generator: None,
            query_interface_fn: None,
            add_ref_fn: None,
            release_fn: None,
        }
    }
}

impl Generator {
    pub fn include_path<T: AsRef<Path>>(mut self, path: T) -> Self {
        self.include_paths.push(path.as_ref().to_path_buf());
        self
    }

    pub fn skip_type<T: AsRef<str>>(mut self, type_: T) -> Self {
        self.skip_types.insert(type_.as_ref().to_string());
        self
    }

    pub fn skip_types<'a, T: AsRef<[&'a str]>>(mut self, types: T) -> Self {
        self.skip_types
            .extend(types.as_ref().iter().map(|s| s.to_string()));
        self
    }

    pub fn skip_interface_trait<T: AsRef<str>>(mut self, interface: T) -> Self {
        self.skip_interface_traits
            .insert(interface.as_ref().to_string());
        self
    }

    pub fn skip_interface_traits<'a, T: AsRef<[&'a str]>>(mut self, interfaces: T) -> Self {
        self.skip_interface_traits
            .extend(interfaces.as_ref().iter().map(|s| s.to_string()));
        self
    }

    pub fn constant_parser<F>(mut self, f: F) -> Self
    where
        F: Fn(&[String]) -> Option<String> + 'static,
    {
        self.constant_parser = Some(Box::new(f));
        self
    }

    pub fn iid_generator<F>(mut self, f: F) -> Self
    where
        F: Fn(&str) -> String + 'static,
    {
        self.iid_generator = Some(Box::new(f));
        self
    }

    pub fn query_interface_fn<T: AsRef<str>>(mut self, f: T) -> Self {
        self.query_interface_fn = Some(f.as_ref().to_string());
        self
    }

    pub fn add_ref_fn<T: AsRef<str>>(mut self, f: T) -> Self {
        self.add_ref_fn = Some(f.as_ref().to_string());
        self
    }

    pub fn release_fn<T: AsRef<str>>(mut self, f: T) -> Self {
        self.release_fn = Some(f.as_ref().to_string());
        self
    }

    pub fn generate<T: AsRef<str>, W: Write>(
        &self,
        source: T,
        sink: W,
    ) -> Result<(), Box<dyn Error>> {
        if !clang_sys::is_loaded() {
            clang_sys::load()?;
        }

        let mut clang_target = None;
        if let Ok(target) = env::var("TARGET") {
            if target != HOST_TARGET {
                clang_target = Some(rust_to_clang_target(&target));
            }
        }

        let unit = TranslationUnit::new(
            source.as_ref(),
            &self.include_paths,
            clang_target.as_deref(),
        )
        .unwrap();

        let namespace = Namespace::parse(&unit.cursor(), &self)?;

        let mut printer = RustPrinter::new(sink, &self);
        printer.print_namespace(&namespace)?;

        Ok(())
    }
}

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
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

/// Builder struct for configuring and generating bindings.
pub struct Generator {
    pub(crate) include_paths: Vec<PathBuf>,
    pub(crate) target: Option<String>,
    pub(crate) skip_types: HashSet<String>,
    pub(crate) skip_interface_traits: HashSet<String>,
    pub(crate) override_typedef_types: HashMap<String, String>,
    pub(crate) override_constant_values: HashMap<String, String>,
    pub(crate) constant_parser: Option<Box<dyn Fn(&[&str]) -> Option<String>>>,
    pub(crate) iid_generator: Option<Box<dyn Fn(&str) -> String>>,
    pub(crate) query_interface_fn: Option<String>,
    pub(crate) add_ref_fn: Option<String>,
    pub(crate) release_fn: Option<String>,
}

impl Default for Generator {
    fn default() -> Generator {
        Generator {
            include_paths: Vec::new(),
            target: None,
            skip_types: HashSet::new(),
            skip_interface_traits: HashSet::new(),
            override_typedef_types: HashMap::new(),
            override_constant_values: HashMap::new(),
            constant_parser: None,
            iid_generator: None,
            query_interface_fn: None,
            add_ref_fn: None,
            release_fn: None,
        }
    }
}

impl Generator {
    /// Adds `path` to the list of include paths to pass to `libclang`.
    pub fn include_path<T: AsRef<Path>>(mut self, path: T) -> Self {
        self.include_paths.push(path.as_ref().to_path_buf());
        self
    }

    /// Specify the target triple for which bindings should be generated.
    pub fn target<T: AsRef<str>>(mut self, target: T) -> Self {
        self.target = Some(target.as_ref().to_string());
        self
    }

    /// Do not generate bindings for `type_`.
    pub fn skip_type<T: AsRef<str>>(mut self, type_: T) -> Self {
        self.skip_types.insert(type_.as_ref().to_string());
        self
    }

    /// Do not generate bindings for `types`.
    pub fn skip_types<S: AsRef<str>, T: AsRef<[S]>>(mut self, types: T) -> Self {
        self.skip_types
            .extend(types.as_ref().iter().map(|s| s.as_ref().to_string()));
        self
    }

    /// Do not generate an interface trait for `interface`.
    pub fn skip_interface_trait<T: AsRef<str>>(mut self, interface: T) -> Self {
        self.skip_interface_traits
            .insert(interface.as_ref().to_string());
        self
    }

    /// Do not generate interface traits for `interfaces`.
    pub fn skip_interface_traits<'a, T: AsRef<[&'a str]>>(mut self, interfaces: T) -> Self {
        self.skip_interface_traits
            .extend(interfaces.as_ref().iter().map(|s| s.to_string()));
        self
    }

    /// Override the type of `typedef` in the generator's output.
    pub fn override_typedef_type<T: AsRef<str>, U: AsRef<str>>(
        mut self,
        typedef: T,
        type_: U,
    ) -> Self {
        self.override_typedef_types
            .insert(typedef.as_ref().to_string(), type_.as_ref().to_string());
        self
    }

    /// Override the types of typedefs in the generator's output based on the typedef-type pairs in
    /// `typedefs`.
    pub fn override_typedef_types<T, U, I>(mut self, typedefs: I) -> Self
    where
        T: AsRef<str>,
        U: AsRef<str>,
        I: AsRef<[(T, U)]>,
    {
        self.override_typedef_types.extend(
            typedefs
                .as_ref()
                .into_iter()
                .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string())),
        );
        self
    }

    /// Override the value of `constant` in the generator's output.
    pub fn override_constant_value<T: AsRef<str>, U: AsRef<str>>(
        mut self,
        constant: T,
        value: U,
    ) -> Self {
        self.override_constant_values
            .insert(constant.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Override the values of constants in the generator's output based on the constant-value pairs
    /// in `constants`.
    pub fn override_constant_values<T, U, I>(mut self, constants: I) -> Self
    where
        T: AsRef<str>,
        U: AsRef<str>,
        I: AsRef<[(T, U)]>,
    {
        self.override_constant_values.extend(
            constants
                .as_ref()
                .into_iter()
                .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string())),
        );
        self
    }

    /// Registers a callback for parsing constant definitions which `libclang` is not able to
    /// evaluate.
    ///
    /// The callback will be passed a slice of tokens, and its output (if not `None`) will be
    /// included in the generated bindings.
    pub fn constant_parser<F>(mut self, f: F) -> Self
    where
        F: Fn(&[&str]) -> Option<String> + 'static,
    {
        self.constant_parser = Some(Box::new(f));
        self
    }

    /// Registers a callback which should, when given the name of an interface as a string, return
    /// a string containing a Rust expression evaluating to the `Guid` value for that interface.
    pub fn iid_generator<F>(mut self, f: F) -> Self
    where
        F: Fn(&str) -> String + 'static,
    {
        self.iid_generator = Some(Box::new(f));
        self
    }

    /// Registers a function which will be called by the implementations of
    /// `Unknown::query_interface` for generated interface types.
    ///
    /// The function should be in scope where the resulting bindings are placed, and it should have
    /// the same type signature as `Unknown::query_interface`.
    pub fn query_interface_fn<T: AsRef<str>>(mut self, f: T) -> Self {
        self.query_interface_fn = Some(f.as_ref().to_string());
        self
    }

    /// Registers a function which will be called by the implementations of `Unknown::add_ref` for
    /// generated interface types.
    ///
    /// The function should be in scope where the resulting bindings are placed, and it should have
    /// the same type signature as `Unknown::add_ref`.
    pub fn add_ref_fn<T: AsRef<str>>(mut self, f: T) -> Self {
        self.add_ref_fn = Some(f.as_ref().to_string());
        self
    }

    /// Registers a function which will be called by the implementations of `Unknown::release` for
    /// generated interface types.
    ///
    /// The function should be in scope where the resulting bindings are placed, and it should have
    /// the same type signature as `Unknown::release`.
    pub fn release_fn<T: AsRef<str>>(mut self, f: T) -> Self {
        self.release_fn = Some(f.as_ref().to_string());
        self
    }

    /// Generates Rust bindings for the C++ definitions in `source` and outputs them via `sink`.
    pub fn generate<T: AsRef<str>, W: Write>(
        &self,
        source: T,
        sink: W,
    ) -> Result<(), Box<dyn Error>> {
        if !clang_sys::is_loaded() {
            clang_sys::load()?;
        }

        let rust_target = if let Some(target) = &self.target {
            Cow::from(target)
        } else if let Ok(target) = env::var("TARGET") {
            Cow::from(target)
        } else {
            Cow::from(HOST_TARGET)
        };
        let clang_target = rust_to_clang_target(&rust_target);

        let unit = TranslationUnit::new(source.as_ref(), &self.include_paths, Some(&clang_target))?;

        let namespace = Namespace::parse(&unit.cursor(), &self)?;

        let mut printer = RustPrinter::new(sink, &self);
        printer.print_namespace(&namespace)?;

        Ok(())
    }
}

use std::collections::HashSet;
use std::error::Error;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::clang::*;
use crate::parse::*;
use crate::print::*;

pub struct GeneratorOptions {
    pub skip_types: HashSet<String>,
    pub include_paths: Vec<PathBuf>,
}

impl Default for GeneratorOptions {
    fn default() -> GeneratorOptions {
        GeneratorOptions {
            skip_types: HashSet::new(),
            include_paths: Vec::new(),
        }
    }
}

pub struct Generator {
    options: GeneratorOptions,
    source: String,
}

impl Generator {
    pub fn new() -> Self {
        Generator {
            options: GeneratorOptions::default(),
            source: String::new(),
        }
    }

    pub fn skip_type<T: AsRef<str>>(mut self, type_: T) -> Self {
        self.options.skip_types.insert(type_.as_ref().to_string());
        self
    }

    pub fn skip_types<'a, T: AsRef<[&'a str]>>(mut self, types: T) -> Self {
        self.options
            .skip_types
            .extend(types.as_ref().iter().map(|s| s.to_string()));
        self
    }

    pub fn include_path<T: AsRef<Path>>(mut self, path: T) -> Self {
        self.options.include_paths.push(path.as_ref().to_path_buf());
        self
    }

    pub fn source<T: Into<String>>(mut self, source: T) -> Self {
        self.source = source.into();
        self
    }

    pub fn generate<W: Write>(&self, sink: W) -> Result<(), Box<dyn Error>> {
        let unit = TranslationUnit::new(&self.source, &self.options.include_paths).unwrap();

        let namespace = Namespace::parse(&unit.cursor(), &self.options)?;

        let mut printer = RustPrinter::new(sink);
        printer.print_namespace(&namespace)?;

        Ok(())
    }
}

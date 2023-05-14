use std::io::{Result, Write};

use super::parse::Namespace;

pub struct RustPrinter<W> {
    sink: W,
    indent_level: usize,
}

impl<W: Write> RustPrinter<W> {
    pub fn new(sink: W) -> RustPrinter<W> {
        RustPrinter {
            sink,
            indent_level: 0,
        }
    }

    fn indent(&mut self) -> Result<()> {
        for _ in 0..self.indent_level {
            write!(self.sink, "    ")?;
        }

        Ok(())
    }

    pub fn print_namespace(&mut self, namespace: &Namespace) -> Result<()> {
        for class in &namespace.classes {
            self.indent()?;
            writeln!(self.sink, "pub struct {};", &class.name)?;
        }

        for (name, child) in &namespace.children {
            self.indent()?;
            writeln!(self.sink, "pub mod {} {{", name)?;

            self.indent_level += 1;
            self.print_namespace(child)?;
            self.indent_level -= 1;

            self.indent()?;
            writeln!(self.sink, "}}")?;
        }

        Ok(())
    }
}

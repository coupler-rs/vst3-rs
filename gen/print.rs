use std::io::Write;

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

    pub fn print_namespace(&mut self, namespace: &Namespace) {
        for class in &namespace.classes {
            for _ in 0..self.indent_level {
                write!(self.sink, "    ").unwrap();
            }
            writeln!(self.sink, "pub struct {};", &class.name).unwrap();
        }

        for (name, child) in &namespace.children {
            for _ in 0..self.indent_level {
                write!(self.sink, "    ").unwrap();
            }
            writeln!(self.sink, "pub mod {} {{", name).unwrap();

            self.indent_level += 1;
            self.print_namespace(child);
            self.indent_level -= 1;

            for _ in 0..self.indent_level {
                write!(self.sink, "    ").unwrap();
            }
            writeln!(self.sink, "}}").unwrap();
        }
    }
}

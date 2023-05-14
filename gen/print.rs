use std::io::{Result, Write};

use super::parse::{Namespace, Type};

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
        for typedef in &namespace.typedefs {
            self.indent()?;
            write!(self.sink, "pub type {} = ", typedef.name)?;
            self.print_type(&typedef.type_)?;
            writeln!(self.sink, ";")?;
        }

        for struct_ in &namespace.structs {
            self.indent()?;
            writeln!(self.sink, "pub struct {};", struct_.name)?;
        }

        for class in &namespace.classes {
            self.indent()?;
            writeln!(self.sink, "pub struct {};", &class.name)?;
        }

        for (name, child) in &namespace.children {
            self.indent()?;
            writeln!(self.sink, "pub mod {} {{", name)?;

            self.indent_level += 1;

            self.indent()?;
            writeln!(self.sink, "use super::*;")?;

            self.print_namespace(child)?;

            self.indent_level -= 1;

            self.indent()?;
            writeln!(self.sink, "}}")?;
        }

        Ok(())
    }

    fn print_type(&mut self, type_: &Type) -> Result<()> {
        match type_ {
            Type::Void => write!(self.sink, "std::ffi::c_void")?,
            Type::Bool => write!(self.sink, "bool")?,
            Type::Char => write!(self.sink, "std::ffi::c_char")?,
            Type::UChar => write!(self.sink, "std::ffi::c_uchar")?,
            Type::UShort => write!(self.sink, "std::ffi::c_ushort")?,
            Type::UInt => write!(self.sink, "std::ffi::c_uint")?,
            Type::ULong => write!(self.sink, "std::ffi::c_ulong")?,
            Type::ULongLong => write!(self.sink, "std::ffi::c_ulonglong")?,
            Type::SChar => write!(self.sink, "std::ffi::c_schar")?,
            Type::Short => write!(self.sink, "std::ffi::c_short")?,
            Type::Int => write!(self.sink, "std::ffi::c_int")?,
            Type::Long => write!(self.sink, "std::ffi::c_long")?,
            Type::LongLong => write!(self.sink, "std::ffi::c_longlong")?,
            Type::Float => write!(self.sink, "f32")?,
            Type::Double => write!(self.sink, "f64")?,
            Type::Pointer { is_const, pointee } | Type::Reference { is_const, pointee } => {
                if *is_const {
                    write!(self.sink, "*const ")?;
                } else {
                    write!(self.sink, "*mut ")?;
                }
                self.print_type(pointee)?;
            }
            Type::Typedef(name) => write!(self.sink, "{}", name)?,
            Type::Array(size, elem) => {
                write!(self.sink, "[")?;
                self.print_type(elem)?;
                write!(self.sink, "; {size}]")?
            }
        }

        Ok(())
    }
}

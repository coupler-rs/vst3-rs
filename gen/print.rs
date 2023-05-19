use std::collections::HashSet;
use std::io::{self, ErrorKind, Write};

use super::parse::{Namespace, Record, RecordKind, Type};

pub struct RustPrinter<W> {
    sink: W,
    indent_level: usize,
    reserved: HashSet<&'static str>,
}

impl<W: Write> RustPrinter<W> {
    pub fn new(sink: W) -> RustPrinter<W> {
        RustPrinter {
            sink,
            indent_level: 0,
            reserved: HashSet::from(["type"]),
        }
    }

    fn indent(&mut self) -> io::Result<()> {
        for _ in 0..self.indent_level {
            write!(self.sink, "    ")?;
        }

        Ok(())
    }

    pub fn print_namespace(&mut self, namespace: &Namespace) -> io::Result<()> {
        for typedef in &namespace.typedefs {
            self.indent()?;
            write!(self.sink, "pub type {} = ", typedef.name)?;
            self.print_type(&typedef.type_)?;
            writeln!(self.sink, ";")?;
        }

        for record in &namespace.records {
            self.print_record(&record)?;
        }

        for (name, child) in &namespace.children {
            if !child.is_empty() {
                self.indent()?;
                writeln!(self.sink, "pub mod {} {{", name)?;
                self.indent_level += 1;

                self.indent()?;
                writeln!(self.sink, "#[allow(unused_imports)]")?;
                self.indent()?;
                writeln!(self.sink, "use super::*;")?;

                self.print_namespace(child)?;

                self.indent_level -= 1;
                self.indent()?;
                writeln!(self.sink, "}}")?;
            }
        }

        Ok(())
    }

    fn print_record(&mut self, record: &Record) -> io::Result<()> {
        let needs_module = !record.inner.is_empty() || !record.virtual_methods.is_empty();

        if needs_module {
            self.indent()?;
            writeln!(self.sink, "mod __{}_wrapper {{", record.name)?;
            self.indent_level += 1;

            self.indent()?;
            writeln!(self.sink, "#[allow(unused_imports)]")?;
            self.indent()?;
            writeln!(self.sink, "use super::*;")?;

            self.indent()?;
            writeln!(self.sink, "#[allow(unused_imports)]")?;
            self.indent()?;
            writeln!(self.sink, "use super::{}_::*;", record.name)?;

            self.print_record_body(record)?;

            self.indent_level -= 1;
            self.indent()?;
            writeln!(self.sink, "}}")?;

            self.indent()?;
            writeln!(
                self.sink,
                "pub use __{}_wrapper::{};",
                record.name, record.name
            )?;

            self.indent()?;
            writeln!(self.sink, "pub mod {}_ {{", record.name)?;
            self.indent_level += 1;

            self.indent()?;
            writeln!(self.sink, "#[allow(unused_imports)]")?;
            self.indent()?;
            writeln!(self.sink, "use super::*;")?;

            if !record.virtual_methods.is_empty() {
                self.indent()?;
                writeln!(self.sink, "#[repr(C)]")?;
                self.indent()?;
                writeln!(self.sink, "#[derive(Copy, Clone)]")?;
                self.indent()?;
                writeln!(self.sink, "pub struct Vtbl {{")?;
                self.indent_level += 1;

                if record.bases.len() > 1 {
                    return Err(io::Error::new(
                        ErrorKind::Other,
                        format!("type {} has more than one base class", record.name),
                    ));
                }
                if let Some(base) = record.bases.first() {
                    self.indent()?;
                    writeln!(self.sink, "pub base: {base}_::Vtbl,")?;
                }

                for method in &record.virtual_methods {
                    self.indent()?;
                    writeln!(self.sink, "pub {}: fn(", method.name)?;
                    self.indent_level += 1;

                    self.indent()?;
                    writeln!(self.sink, "this: *mut {},", record.name)?;
                    for arg in &method.arguments {
                        self.indent()?;
                        if !arg.name.is_empty() {
                            if self.reserved.contains(&*arg.name) {
                                write!(self.sink, "r#{}: ", arg.name)?;
                            } else {
                                write!(self.sink, "{}: ", arg.name)?;
                            }
                        }
                        self.print_type(&arg.type_)?;
                        writeln!(self.sink, ",")?;
                    }

                    self.indent_level -= 1;
                    self.indent()?;
                    write!(self.sink, ")")?;
                    if let Type::Void = method.result_type {
                    } else {
                        write!(self.sink, " -> ")?;
                        self.print_type(&method.result_type)?;
                    }
                    writeln!(self.sink, ",")?;
                }

                self.indent_level -= 1;
                self.indent()?;
                writeln!(self.sink, "}}")?;
            }

            self.print_namespace(&record.inner)?;

            self.indent_level -= 1;
            self.indent()?;
            writeln!(self.sink, "}}")?;
        } else {
            self.print_record_body(record)?;
        }

        Ok(())
    }

    fn print_record_body(&mut self, record: &Record) -> io::Result<()> {
        self.indent()?;
        writeln!(self.sink, "#[repr(C)]")?;
        self.indent()?;
        writeln!(self.sink, "#[derive(Copy, Clone)]")?;

        self.indent()?;
        match record.kind {
            RecordKind::Struct => {
                writeln!(self.sink, "pub struct {} {{", record.name)?;
            }
            RecordKind::Union => {
                writeln!(self.sink, "pub union {} {{", record.name)?;
            }
        }
        self.indent_level += 1;

        if !record.virtual_methods.is_empty() {
            self.indent()?;
            writeln!(self.sink, "pub vtbl: {}_::Vtbl,", record.name)?;
        }

        let mut anon_counter = 0;
        for field in &record.fields {
            self.indent()?;
            if field.name.is_empty() {
                write!(self.sink, "pub __field{anon_counter}: ")?;
                anon_counter += 1;
            } else if self.reserved.contains(&*field.name) {
                write!(self.sink, "pub r#{}: ", field.name)?;
            } else {
                write!(self.sink, "pub {}: ", field.name)?;
            }

            self.print_type(&field.type_)?;
            writeln!(self.sink, ",")?;
        }

        self.indent_level -= 1;
        self.indent()?;
        writeln!(self.sink, "}}")?;

        Ok(())
    }

    fn print_type(&mut self, type_: &Type) -> io::Result<()> {
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
            Type::Unsigned(size) => match size {
                1 => write!(self.sink, "u8")?,
                2 => write!(self.sink, "u16")?,
                4 => write!(self.sink, "u32")?,
                8 => write!(self.sink, "u64")?,
                _ => {
                    return Err(io::Error::new(
                        ErrorKind::Other,
                        format!("unexpected size {} for unsigned integer", size),
                    ))
                }
            },
            Type::Signed(size) => match size {
                1 => write!(self.sink, "i8")?,
                2 => write!(self.sink, "i16")?,
                4 => write!(self.sink, "i32")?,
                8 => write!(self.sink, "i64")?,
                _ => {
                    return Err(io::Error::new(
                        ErrorKind::Other,
                        format!("unexpected size {} for signed integer", size),
                    ))
                }
            },
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
            Type::Record(name) => write!(self.sink, "{}", name)?,
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

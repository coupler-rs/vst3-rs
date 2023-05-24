use std::collections::HashSet;
use std::io::{self, ErrorKind, Write};

use crate::generator::GeneratorOptions;
use crate::parse::{Method, Namespace, Record, RecordKind, Type, Value};

struct UnnamedRecordScope {
    prefix: String,
    records: Vec<Record>,
}

impl UnnamedRecordScope {
    fn next_name(&self) -> String {
        format!("{}__type{}", self.prefix, self.records.len())
    }

    fn add_record(&mut self, record: Record) {
        self.records.push(record);
    }
}

pub struct RustPrinter<'a, W> {
    sink: W,
    options: &'a GeneratorOptions,
    reserved: HashSet<&'static str>,
    indent_level: usize,
    unnamed_records: Vec<UnnamedRecordScope>,
}

impl<'a, W: Write> RustPrinter<'a, W> {
    pub fn new(sink: W, options: &'a GeneratorOptions) -> RustPrinter<'a, W> {
        RustPrinter {
            sink,
            options: options,
            reserved: HashSet::from(["type"]),
            indent_level: 0,
            unnamed_records: Vec::new(),
        }
    }

    fn indent(&mut self) -> io::Result<()> {
        for _ in 0..self.indent_level {
            write!(self.sink, "    ")?;
        }

        Ok(())
    }

    pub fn print_namespace(&mut self, namespace: &Namespace) -> io::Result<()> {
        self.push_unnamed_records("");

        for typedef in &namespace.typedefs {
            self.indent()?;
            write!(self.sink, "pub type {} = ", typedef.name)?;
            self.print_type(&typedef.type_)?;
            writeln!(self.sink, ";")?;

            if !typedef.inner.is_empty() {
                self.indent()?;
                writeln!(self.sink, "pub mod {}_ {{", typedef.name)?;
                self.indent_level += 1;

                self.indent()?;
                writeln!(self.sink, "#[allow(unused_imports)]")?;
                self.indent()?;
                writeln!(self.sink, "use super::*;")?;

                self.print_namespace(&typedef.inner)?;

                self.indent_level -= 1;
                self.indent()?;
                writeln!(self.sink, "}}")?;
            }
        }

        for record in &namespace.records {
            self.print_record(&record)?;
        }

        for constant in &namespace.constants {
            self.indent()?;
            write!(self.sink, "pub const {}: ", constant.name)?;
            self.print_type(&constant.type_)?;
            match constant.value {
                Value::Signed(value) => writeln!(self.sink, " = {:?};", value)?,
                Value::Unsigned(value) => writeln!(self.sink, " = {:?};", value)?,
                Value::Float(value) => writeln!(self.sink, " = {:?};", value)?,
            }
        }

        for constant in &namespace.unparsed_constants {
            self.indent()?;
            write!(self.sink, "{}", constant)?;
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

        self.pop_unnamed_records()?;

        Ok(())
    }

    fn print_record(&mut self, record: &Record) -> io::Result<()> {
        self.push_unnamed_records(&record.name);

        let needs_module = !record.inner.is_empty();

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
            self.print_interface(record)?;

            self.indent_level -= 1;
            self.indent()?;
            writeln!(self.sink, "}}")?;

            self.indent()?;
            writeln!(self.sink, "pub use __{}_wrapper::*;", record.name)?;

            self.indent()?;
            writeln!(self.sink, "pub mod {}_ {{", record.name)?;
            self.indent_level += 1;

            self.indent()?;
            writeln!(self.sink, "#[allow(unused_imports)]")?;
            self.indent()?;
            writeln!(self.sink, "use super::*;")?;

            self.print_namespace(&record.inner)?;

            self.indent_level -= 1;
            self.indent()?;
            writeln!(self.sink, "}}")?;
        } else {
            self.print_record_body(record)?;
            self.print_interface(record)?;
        }

        self.pop_unnamed_records()?;

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
            writeln!(self.sink, "pub vtbl: *const {}Vtbl,", record.name)?;
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

        self.indent()?;
        writeln!(self.sink, "unsafe impl Send for {} {{}}", record.name)?;
        self.indent()?;
        writeln!(self.sink, "unsafe impl Sync for {} {{}}", record.name)?;

        Ok(())
    }

    fn print_interface(&mut self, record: &Record) -> io::Result<()> {
        if !record.virtual_methods.is_empty() {
            self.indent()?;
            writeln!(self.sink, "#[repr(C)]")?;
            self.indent()?;
            writeln!(self.sink, "#[derive(Copy, Clone)]")?;
            self.indent()?;
            writeln!(self.sink, "pub struct {}Vtbl {{", record.name)?;
            self.indent_level += 1;

            if record.bases.len() > 1 {
                return Err(io::Error::new(
                    ErrorKind::Other,
                    format!("type {} has more than one base class", record.name),
                ));
            }
            if let Some(base) = record.bases.first() {
                self.indent()?;
                writeln!(self.sink, "pub base: {base}Vtbl,")?;
            }

            for method in &record.virtual_methods {
                self.indent()?;
                writeln!(
                    self.sink,
                    "pub {}: unsafe extern \"system\" fn(",
                    method.name
                )?;
                self.indent_level += 1;

                self.indent()?;
                writeln!(self.sink, "this: *mut {},", record.name)?;

                self.print_args(method)?;

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

            self.indent()?;
            writeln!(self.sink, "#[repr(transparent)]")?;
            self.indent()?;
            writeln!(self.sink, "#[derive(Copy, Clone)]")?;
            self.indent()?;
            writeln!(self.sink, "pub struct {0}Ptr(pub *mut {0});", record.name)?;

            if let Some(base) = record.bases.first() {
                self.indent()?;
                writeln!(
                    self.sink,
                    "impl ::std::ops::Deref for {}Ptr {{",
                    record.name
                )?;
                self.indent()?;
                writeln!(self.sink, "    type Target = {}Ptr;", base)?;
                self.indent()?;
                writeln!(self.sink, "    fn deref(&self) -> &Self::Target {{")?;
                self.indent()?;
                writeln!(
                    self.sink,
                    "        unsafe {{ ::std::mem::transmute(self) }}"
                )?;
                self.indent()?;
                writeln!(self.sink, "    }}")?;
                self.indent()?;
                writeln!(self.sink, "}}")?;
            }

            self.indent()?;
            writeln!(self.sink, "impl {}Ptr {{", record.name)?;
            self.indent_level += 1;

            for method in &record.virtual_methods {
                self.indent()?;
                writeln!(self.sink, "pub unsafe fn {}(", method.name)?;
                self.indent_level += 1;

                self.indent()?;
                writeln!(self.sink, "&self,")?;

                self.print_args(method)?;

                self.indent_level -= 1;
                self.indent()?;
                write!(self.sink, ")")?;
                if let Type::Void = method.result_type {
                } else {
                    write!(self.sink, " -> ")?;
                    self.print_type(&method.result_type)?;
                }
                writeln!(self.sink, " {{")?;
                self.indent_level += 1;

                self.indent()?;
                writeln!(self.sink, "((*(*self.0).vtbl).{})(", method.name)?;
                self.indent_level += 1;

                self.indent()?;
                writeln!(self.sink, "self.0,")?;

                let mut unnamed_counter = 0;
                for arg in &method.arguments {
                    self.indent()?;
                    if arg.name.is_empty() {
                        write!(self.sink, "_{unnamed_counter}")?;
                        unnamed_counter += 1;
                    } else {
                        if self.reserved.contains(&*arg.name) {
                            write!(self.sink, "r#{}", arg.name)?;
                        } else {
                            write!(self.sink, "{}", arg.name)?;
                        }
                    }
                    writeln!(self.sink, ",")?;
                }

                self.indent_level -= 1;
                self.indent()?;
                writeln!(self.sink, ")")?;

                self.indent_level -= 1;
                self.indent()?;
                writeln!(self.sink, "}}")?;
            }

            self.indent_level -= 1;
            self.indent()?;
            writeln!(self.sink, "}}")?;

            if !self.options.skip_interface_traits.contains(&record.name) {
                self.indent()?;
                write!(self.sink, "pub trait {}Trait", record.name)?;
                if let Some(base) = record.bases.first() {
                    if !self.options.skip_interface_traits.contains(base) {
                        write!(self.sink, ": {base}Trait")?;
                    }
                }
                writeln!(self.sink, " {{")?;
                self.indent_level += 1;

                for method in &record.virtual_methods {
                    self.indent()?;
                    writeln!(self.sink, "unsafe fn {}(", method.name)?;
                    self.indent_level += 1;

                    self.indent()?;
                    writeln!(self.sink, "&self,")?;

                    self.print_args(method)?;

                    self.indent_level -= 1;
                    self.indent()?;
                    write!(self.sink, ")")?;
                    if let Type::Void = method.result_type {
                    } else {
                        write!(self.sink, " -> ")?;
                        self.print_type(&method.result_type)?;
                    }
                    writeln!(self.sink, ";")?;
                }

                self.indent_level -= 1;
                self.indent()?;
                writeln!(self.sink, "}}")?;
            }
        }

        Ok(())
    }

    fn print_args(&mut self, method: &Method) -> io::Result<()> {
        let mut unnamed_counter = 0;

        for arg in &method.arguments {
            self.indent()?;
            if arg.name.is_empty() {
                write!(self.sink, "_{unnamed_counter}: ")?;
                unnamed_counter += 1;
            } else {
                if self.reserved.contains(&*arg.name) {
                    write!(self.sink, "r#{}: ", arg.name)?;
                } else {
                    write!(self.sink, "{}: ", arg.name)?;
                }
            }
            self.print_type(&arg.type_)?;
            writeln!(self.sink, ",")?;
        }

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
            Type::UnnamedRecord(record) => {
                let scope = self.unnamed_record_scope_mut();
                let name = scope.next_name();
                let mut record = record.clone();
                record.name = name.clone();
                scope.add_record(record);

                write!(self.sink, "{}", name)?;
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

    fn push_unnamed_records(&mut self, prefix: &str) {
        self.unnamed_records.push(UnnamedRecordScope {
            prefix: prefix.to_string(),
            records: Vec::new(),
        });
    }

    fn unnamed_record_scope_mut(&mut self) -> &mut UnnamedRecordScope {
        self.unnamed_records.last_mut().unwrap()
    }

    fn pop_unnamed_records(&mut self) -> io::Result<()> {
        let unnamed_records = self.unnamed_records.pop().unwrap();

        for record in unnamed_records.records {
            self.print_record(&record)?;
        }

        Ok(())
    }
}

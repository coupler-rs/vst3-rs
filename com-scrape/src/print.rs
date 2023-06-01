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

    fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    pub fn print_namespace(&mut self, namespace: &Namespace) -> io::Result<()> {
        self.push_unnamed_records("");

        let indent = self.indent();

        for typedef in &namespace.typedefs {
            let name = &typedef.name;

            write!(self.sink, "{indent}pub type {name} = ")?;
            self.print_type(&typedef.type_)?;
            writeln!(self.sink, ";")?;

            if !typedef.inner.is_empty() {
                writeln!(self.sink, "{indent}pub mod {name}_ {{")?;
                writeln!(self.sink, "{indent}    #[allow(unused_imports)]")?;
                writeln!(self.sink, "{indent}    use super::*;")?;

                self.indent_level += 1;
                self.print_namespace(&typedef.inner)?;
                self.indent_level -= 1;

                writeln!(self.sink, "{indent}}}")?;
            }
        }

        for record in &namespace.records {
            self.print_record(&record)?;
        }

        for constant in &namespace.constants {
            let name = &constant.name;
            write!(self.sink, "{indent}pub const {name}: ")?;
            self.print_type(&constant.type_)?;
            match constant.value {
                Value::Signed(value) => writeln!(self.sink, " = {value:?};")?,
                Value::Unsigned(value) => writeln!(self.sink, " = {value:?};")?,
                Value::Float(value) => writeln!(self.sink, " = {value:?};")?,
            }
        }

        for constant in &namespace.unparsed_constants {
            let indent = self.indent();
            writeln!(self.sink, "{indent}{constant}")?;
        }

        for (name, child) in &namespace.children {
            if !child.is_empty() {
                writeln!(self.sink, "{indent}pub mod {name} {{")?;
                writeln!(self.sink, "{indent}    #[allow(unused_imports)]")?;
                writeln!(self.sink, "{indent}    use super::*;")?;

                self.indent_level += 1;
                self.print_namespace(child)?;
                self.indent_level -= 1;

                writeln!(self.sink, "{indent}}}")?;
            }
        }

        self.pop_unnamed_records()?;

        Ok(())
    }

    fn print_record(&mut self, record: &Record) -> io::Result<()> {
        self.push_unnamed_records(&record.name);

        let needs_module = !record.inner.is_empty();

        let name = &record.name;

        if needs_module {
            let indent = self.indent();

            writeln!(self.sink, "{indent}mod __{name}_wrapper {{")?;
            writeln!(self.sink, "{indent}    #[allow(unused_imports)]")?;
            writeln!(self.sink, "{indent}    use super::*;")?;
            writeln!(self.sink, "{indent}    #[allow(unused_imports)]")?;
            writeln!(self.sink, "{indent}    use super::{name}_::*;")?;

            self.indent_level += 1;
            self.print_record_body(record)?;
            self.print_interface(record)?;
            self.indent_level -= 1;

            writeln!(self.sink, "{indent}}}")?;
            writeln!(self.sink, "{indent}pub use __{name}_wrapper::*;")?;

            writeln!(self.sink, "{indent}pub mod {name}_ {{")?;
            writeln!(self.sink, "{indent}    #[allow(unused_imports)]")?;
            writeln!(self.sink, "{indent}    use super::*;")?;

            self.indent_level += 1;
            self.print_namespace(&record.inner)?;
            self.indent_level -= 1;

            writeln!(self.sink, "{indent}}}")?;
        } else {
            self.print_record_body(record)?;
            self.print_interface(record)?;
        }

        self.pop_unnamed_records()?;

        Ok(())
    }

    fn print_record_body(&mut self, record: &Record) -> io::Result<()> {
        let indent = self.indent();
        let name = &record.name;
        let record_kind = match record.kind {
            RecordKind::Struct => "struct",
            RecordKind::Union => "union",
        };

        writeln!(self.sink, "{indent}#[repr(C)]")?;
        writeln!(self.sink, "{indent}#[derive(Copy, Clone)]")?;
        writeln!(self.sink, "{indent}pub {record_kind} {name} {{")?;

        if !record.virtual_methods.is_empty() {
            writeln!(self.sink, "{indent}    pub vtbl: *const {name}Vtbl,")?;
        }

        let mut anon_counter = 0;
        for field in &record.fields {
            let field_name = &field.name;
            if field.name.is_empty() {
                write!(self.sink, "{indent}    pub __field{anon_counter}: ")?;
                anon_counter += 1;
            } else if self.reserved.contains(&*field.name) {
                write!(self.sink, "{indent}    pub r#{field_name}: ")?;
            } else {
                write!(self.sink, "{indent}    pub {field_name}: ")?;
            }
            self.print_type(&field.type_)?;
            writeln!(self.sink, ",")?;
        }

        writeln!(self.sink, "{indent}}}")?;
        writeln!(self.sink, "{indent}unsafe impl Send for {name} {{}}")?;
        writeln!(self.sink, "{indent}unsafe impl Sync for {name} {{}}")?;

        Ok(())
    }

    fn print_interface(&mut self, record: &Record) -> io::Result<()> {
        if !record.virtual_methods.is_empty() {
            let indent = self.indent();
            let name = &record.name;

            if record.bases.len() > 1 {
                return Err(io::Error::new(
                    ErrorKind::Other,
                    format!("type {name} has more than one base class"),
                ));
            }

            writeln!(
                self.sink,
                "{indent}unsafe impl ::com_scrape_types::Inherits<{name}> for {name} {{}}"
            )?;
            let mut bases = &record.bases;
            while let Some(base) = bases.first() {
                let base_name = &base.name;
                writeln!(
                    self.sink,
                    "{indent}unsafe impl ::com_scrape_types::Inherits<{base_name}> for {name} {{}}"
                )?;
                bases = &base.bases;
            }

            let iid_generator = self.options.iid_generator.as_ref().ok_or_else(|| {
                io::Error::new(ErrorKind::Other, "no value provided for iid_generator")
            })?;
            let iid_string = iid_generator(name);
            let query_interface_fn = self.options.query_interface_fn.as_ref().ok_or_else(|| {
                io::Error::new(ErrorKind::Other, "no value provided for query_interface_fn")
            })?;
            let add_ref_fn = self.options.add_ref_fn.as_ref().ok_or_else(|| {
                io::Error::new(ErrorKind::Other, "no value provided for add_ref_fn")
            })?;
            let release_fn = self.options.release_fn.as_ref().ok_or_else(|| {
                io::Error::new(ErrorKind::Other, "no value provided for release_fn")
            })?;
            #[rustfmt::skip]
            {
                writeln!(self.sink, "{indent}impl ::com_scrape_types::Unknown for {name} {{")?;
                writeln!(self.sink, "{indent}    #[inline]")?;
                writeln!(self.sink, "{indent}    unsafe fn query_interface(this: *mut Self, iid: &Guid) -> Option<*mut c_void> {{")?;
                writeln!(self.sink, "{indent}        {query_interface_fn}(this as *mut c_void, iid)")?;
                writeln!(self.sink, "{indent}    }}")?;
                writeln!(self.sink, "{indent}    #[inline]")?;
                writeln!(self.sink, "{indent}    unsafe fn add_ref(this: *mut Self) -> usize {{")?;
                writeln!(self.sink, "{indent}        {add_ref_fn}(this as *mut c_void)")?;
                writeln!(self.sink, "{indent}    }}")?;
                writeln!(self.sink, "{indent}    #[inline]")?;
                writeln!(self.sink, "{indent}    unsafe fn release(this: *mut Self) -> usize {{")?;
                writeln!(self.sink, "{indent}        {release_fn}(this as *mut c_void)")?;
                writeln!(self.sink, "{indent}    }}")?;
                writeln!(self.sink, "{indent}}}")?;

                writeln!(self.sink, "{indent}unsafe impl ::com_scrape_types::Interface for {name} {{")?;
                writeln!(self.sink, "{indent}    const IID: ::com_scrape_types::Guid = {iid_string};")?;
                writeln!(self.sink, "{indent}    #[inline]")?;
                writeln!(self.sink, "{indent}    fn inherits(iid: &Guid) -> bool {{")?;
                write!(self.sink, "{indent}        iid == &Self::IID")?;
                if let Some(base) = bases.first() {
                    let base_name = &base.name;
                    write!(self.sink, " || {base_name}::inherits(iid)")?;
                }
                writeln!(self.sink, "")?;
                writeln!(self.sink, "{indent}    }}")?;
                writeln!(self.sink, "{indent}}}")?;
            };

            writeln!(self.sink, "{indent}#[repr(C)]")?;
            writeln!(self.sink, "{indent}#[derive(Copy, Clone)]")?;
            writeln!(self.sink, "{indent}pub struct {name}Vtbl {{")?;

            if let Some(base) = record.bases.first() {
                let base_name = &base.name;
                writeln!(self.sink, "{indent}    pub base: {base_name}Vtbl,")?;
            }

            for method in &record.virtual_methods {
                let method_name = &method.name;
                writeln!(
                    self.sink,
                    "{indent}    pub {method_name}: unsafe extern \"system\" fn("
                )?;

                writeln!(self.sink, "{indent}        this: *mut {name},")?;

                self.indent_level += 2;
                self.print_args(method)?;
                self.indent_level -= 2;

                write!(self.sink, "{indent}    )")?;
                if let Type::Void = method.result_type {
                } else {
                    write!(self.sink, " -> ")?;
                    self.print_type(&method.result_type)?;
                }
                writeln!(self.sink, ",")?;
            }

            writeln!(self.sink, "{indent}}}")?;

            if !self.options.skip_interface_traits.contains(&record.name) {
                write!(self.sink, "{indent}pub trait {name}Trait")?;
                let mut bases = &record.bases;
                while let Some(base) = bases.first() {
                    if !self.options.skip_interface_traits.contains(&base.name) {
                        let base_name = &base.name;
                        write!(self.sink, ": {base_name}Trait")?;
                        break;
                    }
                    bases = &base.bases;
                }
                writeln!(self.sink, " {{")?;

                for method in &record.virtual_methods {
                    let method_name = &method.name;

                    writeln!(self.sink, "{indent}    unsafe fn {method_name}(")?;
                    writeln!(self.sink, "{indent}        &self,")?;

                    self.indent_level += 2;
                    self.print_args(method)?;
                    self.indent_level -= 2;

                    write!(self.sink, "{indent}    )")?;
                    if let Type::Void = method.result_type {
                    } else {
                        write!(self.sink, " -> ")?;
                        self.print_type(&method.result_type)?;
                    }
                    writeln!(self.sink, ";")?;
                }

                writeln!(self.sink, "{indent}}}")?;

                writeln!(self.sink, "{indent}impl<P> {name}Trait for P")?;
                writeln!(self.sink, "{indent}where")?;
                writeln!(self.sink, "{indent}    P: ::com_scrape_types::SmartPtr,")?;
                writeln!(
                    self.sink,
                    "{indent}    P::Target: ::com_scrape_types::Inherits<{name}>,"
                )?;
                let mut bases = &record.bases;
                while let Some(base) = bases.first() {
                    if !self.options.skip_interface_traits.contains(&base.name) {
                        let base_name = &base.name;
                        writeln!(
                            self.sink,
                            "{indent}    P::Target: ::com_scrape_types::Inherits<{base_name}>,"
                        )?;
                    }
                    bases = &base.bases;
                }
                writeln!(self.sink, "{indent}{{")?;

                for method in &record.virtual_methods {
                    let method_name = &method.name;

                    writeln!(self.sink, "{indent}    #[inline]")?;
                    writeln!(self.sink, "{indent}    unsafe fn {method_name}(")?;
                    writeln!(self.sink, "{indent}        &self,")?;

                    self.indent_level += 2;
                    self.print_args(method)?;
                    self.indent_level -= 2;

                    write!(self.sink, "{indent}    )")?;
                    if let Type::Void = method.result_type {
                    } else {
                        write!(self.sink, " -> ")?;
                        self.print_type(&method.result_type)?;
                    }
                    writeln!(self.sink, " {{")?;
                    writeln!(
                        self.sink,
                        "{indent}        let ptr = self.ptr() as *mut {name};"
                    )?;
                    writeln!(self.sink, "{indent}        ((*(*ptr).vtbl).{method_name})(")?;
                    writeln!(self.sink, "{indent}            ptr,")?;

                    let mut unnamed_counter = 0;
                    for arg in &method.arguments {
                        let arg_name = &arg.name;
                        if arg.name.is_empty() {
                            writeln!(self.sink, "{indent}            _{unnamed_counter},")?;
                            unnamed_counter += 1;
                        } else if self.reserved.contains(&*arg.name) {
                            writeln!(self.sink, "{indent}            r#{arg_name},")?;
                        } else {
                            writeln!(self.sink, "{indent}            {arg_name},")?;
                        }
                    }

                    writeln!(self.sink, "{indent}        )")?;
                    writeln!(self.sink, "{indent}    }}")?;
                }

                writeln!(self.sink, "{indent}}}")?;
            }
        }

        Ok(())
    }

    fn print_args(&mut self, method: &Method) -> io::Result<()> {
        let mut unnamed_counter = 0;

        let indent = self.indent();

        for arg in &method.arguments {
            let arg_name = &arg.name;
            if arg.name.is_empty() {
                write!(self.sink, "{indent}_{unnamed_counter}: ")?;
                unnamed_counter += 1;
            } else if self.reserved.contains(&*arg.name) {
                write!(self.sink, "{indent}r#{arg_name}: ")?;
            } else {
                write!(self.sink, "{indent}{arg_name}: ")?;
            }
            self.print_type(&arg.type_)?;
            writeln!(self.sink, ",")?;
        }

        Ok(())
    }

    fn print_type(&mut self, type_: &Type) -> io::Result<()> {
        match type_ {
            Type::Void => write!(self.sink, "::std::ffi::c_void")?,
            Type::Bool => write!(self.sink, "bool")?,
            Type::Char => write!(self.sink, "::std::ffi::c_char")?,
            Type::UChar => write!(self.sink, "::std::ffi::c_uchar")?,
            Type::UShort => write!(self.sink, "::std::ffi::c_ushort")?,
            Type::UInt => write!(self.sink, "::std::ffi::c_uint")?,
            Type::ULong => write!(self.sink, "::std::ffi::c_ulong")?,
            Type::ULongLong => write!(self.sink, "::std::ffi::c_ulonglong")?,
            Type::SChar => write!(self.sink, "::std::ffi::c_schar")?,
            Type::Short => write!(self.sink, "::std::ffi::c_short")?,
            Type::Int => write!(self.sink, "::std::ffi::c_int")?,
            Type::Long => write!(self.sink, "::std::ffi::c_long")?,
            Type::LongLong => write!(self.sink, "::std::ffi::c_longlong")?,
            Type::Unsigned(size) => match size {
                1 => write!(self.sink, "u8")?,
                2 => write!(self.sink, "u16")?,
                4 => write!(self.sink, "u32")?,
                8 => write!(self.sink, "u64")?,
                _ => {
                    return Err(io::Error::new(
                        ErrorKind::Other,
                        format!("unexpected size {size} for unsigned integer"),
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
                        format!("unexpected size {size} for signed integer"),
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
            Type::Record(name) => write!(self.sink, "{name}")?,
            Type::UnnamedRecord(record) => {
                let scope = self.unnamed_record_scope_mut();
                let name = scope.next_name();
                let mut record = record.clone();
                record.name = name.clone();
                scope.add_record(record);

                write!(self.sink, "{name}")?;
            }
            Type::Typedef(name) => write!(self.sink, "{name}")?,
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

use std::collections::{BTreeMap, HashSet};
use std::error::Error;

use super::clang;
use clang::*;

struct Parser {
    namespace_stack: Vec<String>,
    skip_list: HashSet<String>,
    namespace: Namespace,
}

impl Parser {
    fn new(skip_list: &[&str]) -> Parser {
        let skip_list = skip_list
            .iter()
            .map(|s| s.to_string())
            .collect::<HashSet<_>>();

        Parser {
            namespace_stack: Vec::new(),
            skip_list,
            namespace: Namespace::new(),
        }
    }

    fn current_namespace(&mut self) -> &mut Namespace {
        let mut namespace = &mut self.namespace;
        for name in &self.namespace_stack {
            if !namespace.children.contains_key(name) {
                namespace.children.insert(name.clone(), Namespace::new());
            }
            namespace = namespace.children.get_mut(name).unwrap();
        }
        namespace
    }

    fn visit(&mut self, cursor: &Cursor) -> Result<(), Box<dyn Error>> {
        if cursor.is_in_system_header() {
            return Ok(());
        }

        if self.skip_list.contains(cursor.name().to_str().unwrap()) {
            return Ok(());
        }

        match cursor.kind() {
            CursorKind::Namespace => {
                let name = cursor.name();
                let name_str = name.to_str().unwrap();

                // Skip the contents of unnamed namespaces
                if name_str.len() == 0 {
                    return Ok(());
                }

                self.namespace_stack.push(name_str.to_string());

                cursor.visit_children(|cursor| self.visit(cursor))?;

                self.namespace_stack.pop();
            }
            CursorKind::TypedefDecl | CursorKind::TypeAliasDecl => {
                let typedef = cursor.type_().unwrap();
                let name = typedef.typedef_name();

                let type_ = Type::parse(cursor.typedef_underlying_type().unwrap());
                if type_.is_err() {
                    let underlying_type = cursor.typedef_underlying_type().unwrap();
                    return Err(format!(
                        "could not parse typedef {} = {}",
                        typedef.name().to_str().unwrap(),
                        underlying_type.name().to_str().unwrap(),
                    ).into());
                }

                self.current_namespace().typedefs.push(Typedef {
                    name: name.unwrap().to_str().unwrap().to_string(),
                    type_: type_.unwrap(),
                });
            }
            CursorKind::StructDecl | CursorKind::UnionDecl | CursorKind::ClassDecl => {
                if cursor.is_definition() {
                    // Skip unnamed records here, as Record::parse will take care of them
                    if !cursor.name().to_str().unwrap().is_empty() {
                        let record = Record::parse(cursor.type_().unwrap()).unwrap();
                        self.current_namespace().records.push(record);
                    }

                    cursor.visit_children(|cursor| self.visit(cursor))?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Namespace {
    pub children: BTreeMap<String, Namespace>,
    pub typedefs: Vec<Typedef>,
    pub records: Vec<Record>,
}

impl Namespace {
    pub fn new() -> Namespace {
        Namespace {
            children: BTreeMap::new(),
            typedefs: Vec::new(),
            records: Vec::new(),
        }
    }

    pub fn parse(cursor: &Cursor, skip_list: &[&str]) -> Result<Namespace, Box<dyn Error>> {
        let mut parser = Parser::new(skip_list);

        cursor.visit_children(|cursor| parser.visit(cursor))?;

        Ok(parser.namespace)
    }
}

#[derive(Clone, Debug)]
pub struct Typedef {
    pub name: String,
    pub type_: Type,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RecordKind {
    Struct,
    Union,
}

#[derive(Clone, Debug)]
pub struct Record {
    pub name: String,
    pub kind: RecordKind,
    pub fields: Vec<Field>,
    pub bases: Vec<String>,
    pub virtual_methods: Vec<Method>,
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub type_: Type,
}

impl Record {
    fn parse(record: clang::Type) -> Result<Record, Box<dyn Error>> {
        let decl = record.declaration();
        let name = decl.name().to_str().unwrap().to_string();
        let kind = match decl.kind() {
            CursorKind::StructDecl | CursorKind::ClassDecl => RecordKind::Struct,
            CursorKind::UnionDecl => RecordKind::Union,
            _ => unreachable!(),
        };

        let mut fields = Vec::new();
        let mut bases = Vec::new();
        let mut virtual_methods = Vec::new();
        decl.visit_children(|cursor| -> Result<(), Box<dyn Error>> {
            match cursor.kind() {
                // Check for UnionDecl to handle anonymous unions
                CursorKind::FieldDecl | CursorKind::UnionDecl => {
                    let type_ = Type::parse(cursor.type_().unwrap());

                    if type_.is_err() {
                        return Err(format!(
                            "could not parse field {}: {}",
                            cursor.name().to_str().unwrap(),
                            cursor.type_().unwrap().name().to_str().unwrap(),
                        )
                        .into());
                    }

                    fields.push(Field {
                        name: cursor.name().to_str().unwrap().to_string(),
                        type_: type_.unwrap(),
                    });
                }
                CursorKind::CxxMethod => {
                    if cursor.is_virtual() {
                        let mut arguments = Vec::new();

                        for i in 0..cursor.num_arguments().unwrap() {
                            let arg = cursor.argument(i).unwrap();

                            let arg_type = Type::parse(arg.type_().unwrap());
                            arguments.push(Argument {
                                name: arg.name().to_str().unwrap().to_string(),
                                type_: arg_type.unwrap_or(Type::Void),
                            });
                        }

                        let result_type = Type::parse(cursor.result_type().unwrap()).unwrap();

                        virtual_methods.push(Method {
                            name: cursor.name().to_str().unwrap().to_string(),
                            arguments,
                            result_type,
                        });
                    }
                }
                CursorKind::CxxBaseSpecifier => {
                    bases.push(cursor.type_().unwrap().name().to_str().unwrap().to_string());
                }
                _ => {}
            }

            Ok(())
        })?;

        Ok(Record {
            name,
            kind,
            fields,
            bases,
            virtual_methods,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Method {
    pub name: String,
    pub arguments: Vec<Argument>,
    pub result_type: Type,
}

#[derive(Clone, Debug)]
pub struct Argument {
    pub name: String,
    pub type_: Type,
}

#[derive(Clone, Debug)]
pub enum Type {
    Void,
    Bool,
    Char,
    UChar,
    UShort,
    UInt,
    ULong,
    ULongLong,
    SChar,
    Short,
    Int,
    Long,
    LongLong,
    Unsigned(usize),
    #[allow(dead_code)]
    Signed(usize),
    Float,
    Double,
    Pointer {
        is_const: bool,
        pointee: Box<Type>,
    },
    Reference {
        is_const: bool,
        pointee: Box<Type>,
    },
    Record(String),
    UnnamedRecord(Record),
    Typedef(String),
    Array(usize, Box<Type>),
}

impl Type {
    fn parse(type_: clang::Type) -> Result<Type, Box<dyn Error>> {
        match type_.kind() {
            TypeKind::Void => Ok(Type::Void),
            TypeKind::Bool => Ok(Type::Bool),
            TypeKind::Char_U | TypeKind::Char_S => Ok(Type::Char),
            TypeKind::UChar => Ok(Type::UChar),
            TypeKind::UShort => Ok(Type::UShort),
            TypeKind::UInt => Ok(Type::UInt),
            TypeKind::SChar => Ok(Type::SChar),
            TypeKind::Char16 => Ok(Type::Short),
            TypeKind::WChar => Ok(Type::Unsigned(type_.size())),
            TypeKind::ULong => Ok(Type::ULong),
            TypeKind::ULongLong => Ok(Type::ULongLong),
            TypeKind::Short => Ok(Type::Short),
            TypeKind::Int => Ok(Type::Int),
            TypeKind::Long => Ok(Type::Long),
            TypeKind::LongLong => Ok(Type::LongLong),
            TypeKind::Float => Ok(Type::Float),
            TypeKind::Double => Ok(Type::Double),
            TypeKind::Pointer => {
                let pointee = type_.pointee().unwrap();
                Ok(Type::Pointer {
                    is_const: pointee.is_const(),
                    pointee: Box::new(Type::parse(pointee)?),
                })
            }
            TypeKind::LValueReference => {
                let pointee = type_.pointee().unwrap();
                Ok(Type::Reference {
                    is_const: pointee.is_const(),
                    pointee: Box::new(Type::parse(pointee)?),
                })
            }
            TypeKind::Record => {
                let decl = type_.declaration();
                let name = decl.name().to_str().unwrap().to_string();
                if name.is_empty() {
                    Ok(Type::UnnamedRecord(Record::parse(type_)?))
                } else {
                    Ok(Type::Record(name))
                }
            }
            // TypeKind::Enum,
            TypeKind::Typedef => {
                // Skip typedef declarations that are found in system headers
                let declaration = type_.declaration();
                if declaration.is_in_system_header() {
                    let underlying_type = declaration.typedef_underlying_type().unwrap();
                    return Ok(Type::parse(underlying_type)?);
                }

                let name = type_.typedef_name().unwrap().to_str().unwrap().to_string();
                Ok(Type::Typedef(name))
            }
            TypeKind::ConstantArray => {
                let size = type_.array_size().unwrap();
                let element_type = Type::parse(type_.array_element_type().unwrap())?;
                Ok(Type::Array(size, Box::new(element_type)))
            }
            _ => Err(format!("unhandled type kind {:?}", type_.kind()).into()),
        }
    }
}

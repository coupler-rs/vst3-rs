use std::collections::{HashMap, HashSet};

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

    fn visit(&mut self, cursor: &Cursor) {
        if cursor.is_in_system_header() {
            return;
        }

        match cursor.kind() {
            CursorKind::Namespace => {
                let name = cursor.name();
                let name_str = name.to_str().unwrap();

                // Skip the contents of unnamed namespaces
                if name_str.len() == 0 {
                    return;
                }

                self.namespace_stack.push(name_str.to_string());

                cursor.visit_children(|cursor| {
                    self.visit(cursor);
                });

                self.namespace_stack.pop();
            }
            CursorKind::TypedefDecl | CursorKind::TypeAliasDecl => {
                let typedef = cursor.type_().unwrap();
                let name = typedef.typedef_name();

                let type_ = Type::parse(cursor.typedef_underlying_type().unwrap());
                if type_.is_none() {
                    let underlying_type = cursor.typedef_underlying_type().unwrap();
                    panic!(
                        "could not parse typedef {} = {}",
                        typedef.name().to_str().unwrap(),
                        underlying_type.name().to_str().unwrap(),
                    );
                }

                self.current_namespace().typedefs.push(Typedef {
                    name: name.unwrap().to_str().unwrap().to_string(),
                    type_: type_.unwrap(),
                });
            }
            CursorKind::StructDecl => {
                if cursor.is_definition() {
                    let name = cursor.name();
                    self.current_namespace().structs.push(Struct {
                        name: name.to_str().unwrap().to_string(),
                    });
                }
            }
            CursorKind::ClassDecl => {
                if cursor.is_definition() {
                    let name = cursor.name();
                    let name_str = name.to_str().unwrap();
                    if !self.skip_list.contains(name_str) {
                        self.current_namespace().classes.push(Class {
                            name: name_str.to_string(),
                        });
                    }
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct Namespace {
    pub children: HashMap<String, Namespace>,
    pub typedefs: Vec<Typedef>,
    pub structs: Vec<Struct>,
    pub classes: Vec<Class>,
}

impl Namespace {
    pub fn new() -> Namespace {
        Namespace {
            children: HashMap::new(),
            typedefs: Vec::new(),
            structs: Vec::new(),
            classes: Vec::new(),
        }
    }

    pub fn parse(cursor: &Cursor, skip_list: &[&str]) -> Namespace {
        let mut parser = Parser::new(skip_list);

        cursor.visit_children(|cursor| {
            parser.visit(cursor);
        });

        parser.namespace
    }
}

#[derive(Debug)]
pub struct Typedef {
    pub name: String,
    pub type_: Type,
}

#[derive(Debug)]
pub struct Struct {
    pub name: String,
}

#[derive(Debug)]
pub struct Class {
    pub name: String,
}

#[derive(Debug)]
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
    Float,
    Double,
    Pointer { is_const: bool, pointee: Box<Type> },
    Reference { is_const: bool, pointee: Box<Type> },
    Record(String),
    Typedef(String),
    Array(usize, Box<Type>),
}

impl Type {
    fn parse(type_: clang::Type) -> Option<Type> {
        match type_.kind() {
            TypeKind::Void => Some(Type::Void),
            TypeKind::Bool => Some(Type::Bool),
            TypeKind::Char_U | TypeKind::Char_S => Some(Type::Char),
            TypeKind::UChar => Some(Type::UChar),
            TypeKind::UShort => Some(Type::UShort),
            TypeKind::UInt => Some(Type::UInt),
            TypeKind::SChar => Some(Type::SChar),
            TypeKind::Char16 => Some(Type::Short),
            TypeKind::ULong => Some(Type::ULong),
            TypeKind::ULongLong => Some(Type::ULongLong),
            TypeKind::Short => Some(Type::Short),
            TypeKind::Int => Some(Type::Int),
            TypeKind::Long => Some(Type::Long),
            TypeKind::LongLong => Some(Type::LongLong),
            TypeKind::Float => Some(Type::Float),
            TypeKind::Double => Some(Type::Double),
            TypeKind::Pointer => {
                let pointee = type_.pointee().unwrap();
                Some(Type::Pointer {
                    is_const: pointee.is_const(),
                    pointee: Box::new(Type::parse(pointee)?),
                })
            }
            TypeKind::LValueReference => {
                let pointee = type_.pointee().unwrap();
                Some(Type::Reference {
                    is_const: pointee.is_const(),
                    pointee: Box::new(Type::parse(pointee)?),
                })
            }
            TypeKind::Record => {
                let decl = type_.declaration();
                Some(Type::Record(decl.name().to_str().unwrap().to_string()))
            }
            // TypeKind::Enum,
            TypeKind::Typedef => {
                // Skip typedef declarations that are found in system headers
                let declaration = type_.declaration();
                if declaration.is_in_system_header() {
                    let underlying_type = declaration.typedef_underlying_type().unwrap();
                    return Some(Type::parse(underlying_type)?);
                }

                let name = type_.typedef_name().unwrap().to_str().unwrap().to_string();
                Some(Type::Typedef(name))
            }
            TypeKind::ConstantArray => {
                let size = type_.array_size().unwrap();
                let element_type = Type::parse(type_.array_element_type().unwrap())?;
                Some(Type::Array(size, Box::new(element_type)))
            }
            _ => None,
        }
    }
}

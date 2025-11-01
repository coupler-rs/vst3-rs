use std::collections::BTreeMap;
use std::error::Error;

use crate::clang::{self, *};
use crate::Generator;

#[derive(Clone, Debug)]
pub struct Namespace {
    pub children: BTreeMap<String, Namespace>,
    pub typedefs: Vec<Typedef>,
    pub records: Vec<Record>,
    pub extern_records: Vec<ExternRecord>,
    pub constants: Vec<Constant>,
    pub unparsed_constants: Vec<String>,
}

impl Namespace {
    pub fn new() -> Namespace {
        Namespace {
            children: BTreeMap::new(),
            typedefs: Vec::new(),
            records: Vec::new(),
            extern_records: Vec::new(),
            constants: Vec::new(),
            unparsed_constants: Vec::new(),
        }
    }

    pub fn parse(cursor: &Cursor, options: &Generator) -> Result<Namespace, Box<dyn Error>> {
        let mut parser = Parser::new(options);
        let mut namespace = Namespace::new();

        cursor.visit_children(|cursor| parser.visit(&mut namespace, cursor))?;

        Ok(namespace)
    }

    pub fn is_empty(&self) -> bool {
        self.typedefs.is_empty()
            && self.records.is_empty()
            && self.constants.is_empty()
            && self.unparsed_constants.is_empty()
            && self.children.values().all(|child| child.is_empty())
    }
}

#[derive(Clone, Debug)]
pub struct Typedef {
    pub name: String,
    pub type_: Type,
    pub inner: Namespace,
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
    pub bases: Vec<Base>,
    pub virtual_methods: Vec<Method>,
    pub inner: Namespace,
}

#[derive(Clone, Debug)]
pub struct ExternRecord {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Base {
    pub name: String,
    pub bases: Vec<Base>,
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: Option<String>,
    pub type_: Type,
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
pub struct Constant {
    pub name: String,
    pub type_: Type,
    pub value: Value,
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
    #[allow(unused)]
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

#[derive(Clone, Debug)]
pub enum Value {
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    Str(String),
}

struct Parser<'a> {
    options: &'a Generator,
}

impl<'a> Parser<'a> {
    fn new(options: &'a Generator) -> Parser<'a> {
        Parser { options }
    }

    fn visit(&mut self, namespace: &mut Namespace, cursor: &Cursor) -> Result<(), Box<dyn Error>> {
        if cursor.is_in_system_header() {
            return Ok(());
        }

        if self
            .options
            .skip_types
            .contains(cursor.name().to_str().unwrap())
        {
            return Ok(());
        }

        match cursor.kind() {
            CursorKind::Namespace => {
                // Skip the contents of unnamed namespaces
                if cursor.is_anonymous() {
                    return Ok(());
                }

                let name = cursor.name();
                let name_str = name.to_str().unwrap();

                if !namespace.children.contains_key(name_str) {
                    namespace
                        .children
                        .insert(name_str.to_string(), Namespace::new());
                }
                let child_namespace = namespace.children.get_mut(name_str).unwrap();
                cursor.visit_children(|cursor| self.visit(child_namespace, cursor))?;
            }
            CursorKind::TypedefDecl | CursorKind::TypeAliasDecl => {
                let typedef = cursor.type_().unwrap();
                let name = typedef.typedef_name();

                let type_ =
                    self.parse_type(cursor.typedef_underlying_type().unwrap(), cursor.location())?;

                namespace.typedefs.push(Typedef {
                    name: name.unwrap().to_str().unwrap().to_string(),
                    type_,
                    inner: Namespace::new(),
                });
            }
            CursorKind::EnumDecl => {
                let int_type =
                    self.parse_type(cursor.enum_integer_type().unwrap(), cursor.location())?;

                let canonical_type = cursor.enum_integer_type().unwrap().canonical_type();
                let signed = match canonical_type.kind() {
                    TypeKind::Char_U
                    | TypeKind::UChar
                    | TypeKind::UShort
                    | TypeKind::UInt
                    | TypeKind::ULong
                    | TypeKind::ULongLong => false,
                    TypeKind::Char_S
                    | TypeKind::SChar
                    | TypeKind::Short
                    | TypeKind::Int
                    | TypeKind::Long
                    | TypeKind::LongLong => true,
                    _ => return Err(format!("unhandled enum type {:?}", int_type).into()),
                };

                let mut constants = Vec::new();
                cursor.visit_children(|cursor| -> Result<(), Box<dyn Error>> {
                    match cursor.kind() {
                        CursorKind::EnumConstantDecl => {
                            let value = if signed {
                                Value::Signed(cursor.enum_constant_value().unwrap())
                            } else {
                                Value::Unsigned(cursor.enum_constant_value_unsigned().unwrap())
                            };

                            constants.push(Constant {
                                name: cursor.name().to_str().unwrap().to_string(),
                                type_: int_type.clone(),
                                value,
                            });
                        }
                        _ => {}
                    }

                    Ok(())
                })?;

                if cursor.is_anonymous() {
                    namespace.constants.extend(constants);
                } else {
                    let name = cursor.name();
                    let name_str = name.to_str().unwrap();

                    let mut inner = Namespace::new();
                    inner.constants.extend(constants);

                    namespace.typedefs.push(Typedef {
                        name: name_str.to_string(),
                        type_: int_type.clone(),
                        inner,
                    });
                }
            }
            CursorKind::VarDecl => {
                let type_ = cursor.type_().unwrap();
                if type_.is_const() {
                    let eval_result = cursor.evaluate();
                    let value = match eval_result.kind() {
                        EvalResultKind::Int => {
                            if eval_result.is_unsigned_int() {
                                Some(Value::Unsigned(eval_result.as_unsigned()))
                            } else {
                                Some(Value::Signed(eval_result.as_long_long()))
                            }
                        }
                        EvalResultKind::Float => Some(Value::Float(eval_result.as_double())),
                        EvalResultKind::StrLiteral => Some(Value::Str(
                            eval_result.as_str().unwrap().to_str().unwrap().to_string(),
                        )),
                        EvalResultKind::Other => None,
                    };

                    if let Some(value) = value {
                        let type_ = self.parse_type(type_, cursor.location())?;
                        namespace.constants.push(Constant {
                            name: cursor.name().to_str().unwrap().to_string(),
                            type_,
                            value,
                        });
                    } else {
                        if let Some(parser) = &self.options.constant_parser {
                            let tokens = cursor.tokens();

                            let mut token_strings = Vec::new();
                            for i in 0..tokens.len() {
                                let token = tokens.get(i).unwrap();
                                token_strings.push(token.spelling().to_str().unwrap().to_string());
                            }

                            if let Some(result) = parser(&token_strings) {
                                namespace.unparsed_constants.push(result);
                            }
                        }
                    }
                }
            }
            CursorKind::StructDecl | CursorKind::UnionDecl | CursorKind::ClassDecl => {
                if cursor.is_definition() {
                    // Skip unnamed records here, as parse_type will take care of them
                    if !cursor.is_anonymous() {
                        let record = self.parse_record(cursor.type_().unwrap())?;
                        namespace.records.push(record);
                    }
                } else if !cursor.has_definition() {
                    namespace.extern_records.push(ExternRecord {
                        name: cursor.name().to_str().unwrap().to_string(),
                    });
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn parse_record(&mut self, record: clang::Type) -> Result<Record, Box<dyn Error>> {
        let decl = record.declaration();
        let name = decl.name().to_str().unwrap().to_string();
        let kind = match decl.kind() {
            CursorKind::StructDecl | CursorKind::ClassDecl => RecordKind::Struct,
            CursorKind::UnionDecl => RecordKind::Union,
            _ => unreachable!(),
        };

        let mut fields = Vec::new();
        let mut virtual_methods = Vec::new();
        decl.visit_children(|cursor| -> Result<(), Box<dyn Error>> {
            match cursor.kind() {
                // Check for UnionDecl to handle anonymous unions
                CursorKind::FieldDecl | CursorKind::UnionDecl => {
                    let name = if cursor.is_anonymous() {
                        None
                    } else {
                        Some(cursor.name().to_str().unwrap().to_string())
                    };

                    let type_ = self.parse_type(cursor.type_().unwrap(), cursor.location())?;

                    fields.push(Field { name, type_ });
                }
                CursorKind::CxxMethod => {
                    if cursor.is_virtual() {
                        let mut arguments = Vec::new();

                        for i in 0..cursor.num_arguments().unwrap() {
                            let arg = cursor.argument(i).unwrap();

                            let arg_type = arg.type_().unwrap();

                            // Apply array-to-pointer decay for argument types
                            let canonical_type = arg_type.canonical_type();
                            let type_ = if canonical_type.kind() == TypeKind::ConstantArray {
                                let is_const = canonical_type.is_const();
                                let array_type = self.parse_type(arg_type, arg.location())?;
                                Type::Pointer {
                                    is_const,
                                    pointee: Box::new(array_type),
                                }
                            } else {
                                self.parse_type(arg_type, arg.location())?
                            };

                            arguments.push(Argument {
                                name: arg.name().to_str().unwrap().to_string(),
                                type_,
                            });
                        }

                        let result_type = self
                            .parse_type(cursor.result_type().unwrap(), cursor.location())
                            .unwrap();

                        virtual_methods.push(Method {
                            name: cursor.name().to_str().unwrap().to_string(),
                            arguments,
                            result_type,
                        });
                    }
                }
                _ => {}
            }

            Ok(())
        })?;

        let bases = self.collect_bases(&decl)?;

        let mut inner = Namespace::new();
        decl.visit_children(|cursor| self.visit(&mut inner, cursor))?;

        Ok(Record {
            name,
            kind,
            fields,
            bases,
            virtual_methods,
            inner,
        })
    }

    fn collect_bases(&self, decl: &Cursor) -> Result<Vec<Base>, Box<dyn Error>> {
        let mut bases = Vec::new();

        decl.visit_children(|cursor| -> Result<(), Box<dyn Error>> {
            if cursor.kind() == CursorKind::CxxBaseSpecifier {
                let decl = cursor.type_().unwrap().declaration();

                let name = decl.name();
                let transitive_bases = self.collect_bases(&decl)?;

                bases.push(Base {
                    name: name.to_str().unwrap().to_string(),
                    bases: transitive_bases,
                });
            }

            Ok(())
        })?;

        Ok(bases)
    }

    fn parse_type(
        &mut self,
        type_: clang::Type,
        location: Location,
    ) -> Result<Type, Box<dyn Error>> {
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
                    pointee: Box::new(self.parse_type(pointee, location)?),
                })
            }
            TypeKind::LValueReference => {
                let pointee = type_.pointee().unwrap();
                Ok(Type::Reference {
                    is_const: pointee.is_const(),
                    pointee: Box::new(self.parse_type(pointee, location)?),
                })
            }
            TypeKind::Record => {
                let decl = type_.declaration();
                if decl.is_anonymous() {
                    Ok(Type::UnnamedRecord(self.parse_record(type_)?))
                } else {
                    let name = decl.name().to_str().unwrap().to_string();
                    Ok(Type::Record(name))
                }
            }
            TypeKind::Enum => {
                let decl = type_.declaration();
                Ok(Type::Typedef(decl.name().to_str().unwrap().to_string()))
            }
            TypeKind::Typedef => {
                // Skip typedef declarations that are found in system headers
                let declaration = type_.declaration();
                if declaration.is_in_system_header() {
                    let underlying_type = declaration.typedef_underlying_type().unwrap();
                    return Ok(self.parse_type(underlying_type, location)?);
                }

                let name = type_.typedef_name().unwrap().to_str().unwrap().to_string();
                Ok(Type::Typedef(name))
            }
            TypeKind::ConstantArray => {
                let size = type_.array_size().unwrap();
                let element_type =
                    self.parse_type(type_.array_element_type().unwrap(), location)?;
                Ok(Type::Array(size, Box::new(element_type)))
            }
            TypeKind::Elaborated => self.parse_type(type_.named_type().unwrap(), location),
            _ => Err(format!(
                "error at {location}: unhandled type kind {:?}",
                type_.kind()
            )
            .into()),
        }
    }
}

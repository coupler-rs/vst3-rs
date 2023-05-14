use std::collections::{HashMap, HashSet};

use super::clang::*;

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
            CursorKind::TypedefDecl => {
                self.current_namespace().typedefs.push(Typedef {
                    name: cursor.name().to_str().unwrap().to_string(),
                    type_: Type {},
                });
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
    pub classes: Vec<Class>,
}

#[derive(Debug)]
pub struct Class {
    pub name: String,
}

#[derive(Debug)]
pub struct Type {}

#[derive(Debug)]
pub struct Typedef {
    pub name: String,
    pub type_: Type,
}

impl Namespace {
    pub fn new() -> Namespace {
        Namespace {
            children: HashMap::new(),
            typedefs: Vec::new(),
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

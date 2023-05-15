use std::any::Any;
use std::ffi::{c_char, c_int, c_uint, c_void, CStr, CString};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::panic::{catch_unwind, resume_unwind};

use clang_sys::*;

macro_rules! c_str {
    ($str:literal) => {
        concat!($str, "\0").as_ptr() as *const c_char
    };
}

pub struct TranslationUnit {
    index: CXIndex,
    unit: CXTranslationUnit,
}

impl TranslationUnit {
    pub fn new(source: &str, include_path: &str) -> Result<TranslationUnit, ()> {
        let path = CString::new(include_path).unwrap();

        unsafe {
            let index = clang_createIndex(0, 0);

            let args = [
                c_str!("-x"),
                c_str!("c++"),
                c_str!("-I"),
                path.as_ptr() as *const c_char,
            ];

            let filename = c_str!("header.h");
            let mut sources = [CXUnsavedFile {
                Filename: filename,
                Contents: source.as_ptr() as *const c_char,
                Length: source.len() as u64,
            }];

            let mut unit = MaybeUninit::uninit();
            let result = clang_parseTranslationUnit2(
                index,
                filename,
                args.as_ptr(),
                args.len() as c_int,
                sources.as_mut_ptr(),
                sources.len() as u32,
                CXTranslationUnit_None,
                unit.as_mut_ptr(),
            );
            let unit = unit.assume_init();

            if result != CXError_Success {
                clang_disposeIndex(index);
                return Err(());
            }

            Ok(TranslationUnit { index, unit })
        }
    }

    pub fn cursor(&self) -> Cursor {
        unsafe { Cursor::from_raw(clang_getTranslationUnitCursor(self.unit)) }
    }
}

impl Drop for TranslationUnit {
    fn drop(&mut self) {
        unsafe {
            clang_disposeTranslationUnit(self.unit);
            clang_disposeIndex(self.index);
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CursorKind {
    Namespace,
    TypedefDecl,
    TypeAliasDecl,
    StructDecl,
    UnionDecl,
    ClassDecl,
    FieldDecl,
    CxxMethod,
    Other,
}

pub struct Cursor<'a> {
    cursor: CXCursor,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Cursor<'a> {
    unsafe fn from_raw(cursor: CXCursor) -> Cursor<'a> {
        Cursor {
            cursor,
            _marker: PhantomData,
        }
    }

    pub fn kind(&self) -> CursorKind {
        #[allow(non_upper_case_globals)]
        match unsafe { clang_getCursorKind(self.cursor) } {
            CXCursor_Namespace => CursorKind::Namespace,
            CXCursor_TypedefDecl => CursorKind::TypedefDecl,
            CXCursor_TypeAliasDecl => CursorKind::TypeAliasDecl,
            CXCursor_StructDecl => CursorKind::StructDecl,
            CXCursor_UnionDecl => CursorKind::UnionDecl,
            CXCursor_ClassDecl => CursorKind::ClassDecl,
            CXCursor_FieldDecl => CursorKind::FieldDecl,
            CXCursor_CXXMethod => CursorKind::CxxMethod,
            _ => CursorKind::Other,
        }
    }

    pub fn name(&self) -> StringRef<'a> {
        unsafe { StringRef::from_raw(clang_getCursorSpelling(self.cursor)) }
    }

    pub fn is_in_system_header(&self) -> bool {
        unsafe {
            let location = clang_getCursorLocation(self.cursor);
            clang_Location_isInSystemHeader(location) != 0
        }
    }

    pub fn is_definition(&self) -> bool {
        unsafe { clang_equalCursors(self.cursor, clang_getCursorDefinition(self.cursor)) != 0 }
    }

    pub fn type_(&self) -> Option<Type<'a>> {
        let type_ = unsafe { clang_getCursorType(self.cursor) };
        if type_.kind == CXType_Invalid {
            None
        } else {
            Some(unsafe { Type::from_raw(type_) })
        }
    }

    pub fn typedef_underlying_type(&self) -> Option<Type<'a>> {
        let type_ = unsafe { clang_getTypedefDeclUnderlyingType(self.cursor) };
        if type_.kind == CXType_Invalid {
            None
        } else {
            Some(unsafe { Type::from_raw(type_) })
        }
    }

    pub fn num_arguments(&self) -> Option<usize> {
        let num_arguments = unsafe { clang_Cursor_getNumArguments(self.cursor) };

        if num_arguments == -1 {
            None
        } else {
            Some(num_arguments as usize)
        }
    }

    pub fn argument(&self, index: usize) -> Option<Cursor<'a>> {
        unsafe {
            let argument = clang_Cursor_getArgument(self.cursor, index as c_uint);

            if clang_Cursor_isNull(argument) != 0 {
                None
            } else {
                Some(Cursor::from_raw(argument))
            }
        }
    }

    pub fn result_type(&self) -> Option<Type<'a>> {
        let result_type = unsafe { clang_getCursorResultType(self.cursor) };
        if result_type.kind == CXType_Invalid {
            None
        } else {
            Some(unsafe { Type::from_raw(result_type) })
        }
    }

    pub fn is_virtual(&self) -> bool {
        unsafe { clang_CXXMethod_isVirtual(self.cursor) != 0 }
    }

    pub fn visit_children<F>(&self, mut callback: F)
    where
        F: FnMut(&Cursor),
    {
        extern "C" fn visitor(
            cursor: CXCursor,
            _parent: CXCursor,
            client_data: CXClientData,
        ) -> CXChildVisitResult {
            let data = unsafe { &*(client_data as *mut Data) };
            if data.panic.is_some() {
                return CXChildVisit_Break;
            }

            let result = catch_unwind(|| unsafe {
                let data = &mut *(client_data as *mut Data);
                (data.callback)(&Cursor::from_raw(cursor));

                CXChildVisit_Continue
            });

            match result {
                Ok(res) => res,
                Err(err) => {
                    let data = unsafe { &mut *(client_data as *mut Data) };
                    data.panic = Some(err);

                    CXChildVisit_Break
                }
            }
        }

        struct Data<'c> {
            callback: &'c mut dyn FnMut(&Cursor),
            panic: Option<Box<dyn Any + Send + 'static>>,
        }
        let mut data = Data {
            callback: &mut callback,
            panic: None,
        };

        unsafe {
            clang_visitChildren(self.cursor, visitor, &mut data as *mut Data as *mut c_void);
        }

        if let Some(panic) = data.panic {
            resume_unwind(panic);
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TypeKind {
    Void,
    Bool,
    #[allow(non_camel_case_types)]
    Char_U,
    UChar,
    Char16,
    Char32,
    UShort,
    UInt,
    ULong,
    ULongLong,
    #[allow(non_camel_case_types)]
    Char_S,
    SChar,
    WChar,
    Short,
    Int,
    Long,
    LongLong,
    Float,
    Double,
    Pointer,
    LValueReference,
    Record,
    Enum,
    Typedef,
    ConstantArray,
    Other,
}

pub struct Type<'a> {
    type_: CXType,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Type<'a> {
    unsafe fn from_raw(type_: CXType) -> Type<'a> {
        Type {
            type_,
            _marker: PhantomData,
        }
    }

    pub fn kind(&self) -> TypeKind {
        #[allow(non_upper_case_globals)]
        match self.type_.kind {
            CXType_Void => TypeKind::Void,
            CXType_Bool => TypeKind::Bool,
            CXType_Char_U => TypeKind::Char_U,
            CXType_UChar => TypeKind::UChar,
            CXType_Char16 => TypeKind::Char16,
            CXType_Char32 => TypeKind::Char32,
            CXType_UShort => TypeKind::UShort,
            CXType_UInt => TypeKind::UInt,
            CXType_ULong => TypeKind::ULong,
            CXType_ULongLong => TypeKind::ULongLong,
            CXType_Char_S => TypeKind::Char_S,
            CXType_SChar => TypeKind::SChar,
            CXType_WChar => TypeKind::WChar,
            CXType_Short => TypeKind::Short,
            CXType_Int => TypeKind::Int,
            CXType_Long => TypeKind::Long,
            CXType_LongLong => TypeKind::LongLong,
            CXType_Float => TypeKind::Float,
            CXType_Double => TypeKind::Double,
            CXType_Pointer => TypeKind::Pointer,
            CXType_LValueReference => TypeKind::LValueReference,
            CXType_Record => TypeKind::Record,
            CXType_Enum => TypeKind::Enum,
            CXType_Typedef => TypeKind::Typedef,
            CXType_ConstantArray => TypeKind::ConstantArray,
            _ => TypeKind::Other,
        }
    }

    pub fn is_const(&self) -> bool {
        unsafe { clang_isConstQualifiedType(self.type_) != 0 }
    }

    pub fn name(&self) -> StringRef<'a> {
        unsafe { StringRef::from_raw(clang_getTypeSpelling(self.type_)) }
    }

    pub fn declaration(&self) -> Cursor<'a> {
        unsafe { Cursor::from_raw(clang_getTypeDeclaration(self.type_)) }
    }

    pub fn pointee(&self) -> Option<Type<'a>> {
        let pointee = unsafe { clang_getPointeeType(self.type_) };
        if pointee.kind == CXType_Invalid {
            None
        } else {
            Some(unsafe { Type::from_raw(pointee) })
        }
    }

    pub fn typedef_name(&self) -> Option<StringRef<'a>> {
        let name = unsafe { StringRef::from_raw(clang_getTypedefName(self.type_)) };
        if name.to_bytes().is_empty() {
            None
        } else {
            Some(name)
        }
    }

    pub fn array_size(&self) -> Option<usize> {
        let size = unsafe { clang_getArraySize(self.type_) };
        if size == -1 {
            None
        } else {
            Some(size as usize)
        }
    }

    pub fn array_element_type(&self) -> Option<Type<'a>> {
        let element_type = unsafe { clang_getArrayElementType(self.type_) };
        if element_type.kind == CXType_Invalid {
            None
        } else {
            Some(unsafe { Type::from_raw(element_type) })
        }
    }
}

pub struct StringRef<'a> {
    string: CXString,
    _marker: PhantomData<&'a ()>,
}

impl<'a> StringRef<'a> {
    unsafe fn from_raw(string: CXString) -> StringRef<'a> {
        StringRef {
            string,
            _marker: PhantomData,
        }
    }
}

impl<'a> Deref for StringRef<'a> {
    type Target = CStr;

    fn deref(&self) -> &CStr {
        unsafe { CStr::from_ptr(clang_getCString(self.string)) }
    }
}

impl<'a> Drop for StringRef<'a> {
    fn drop(&mut self) {
        unsafe {
            clang_disposeString(self.string);
        }
    }
}

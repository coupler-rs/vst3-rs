use std::any::Any;
use std::error::Error;
use std::ffi::{c_char, c_int, c_longlong, c_uint, c_ulong, c_ulonglong, c_void, CStr, CString};
use std::fmt::Display;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::ptr::NonNull;
use std::{fmt, ptr, slice};

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
    pub fn new(
        source: &str,
        include_paths: &[PathBuf],
        target: Option<&str>,
    ) -> Result<TranslationUnit, Box<dyn Error>> {
        let mut paths_cstrs = Vec::new();
        for include_path in include_paths {
            paths_cstrs.push(CString::new(include_path.to_str().unwrap()).unwrap());
        }

        let mut target_cstr = None;
        if let Some(target) = target {
            target_cstr = Some(CString::new(target).unwrap());
        }

        unsafe {
            let index = clang_createIndex(0, 0);

            let mut args = vec![c_str!("-x"), c_str!("c++")];
            for path in &paths_cstrs {
                args.extend_from_slice(&[c_str!("-I"), path.as_ptr()]);
            }
            if let Some(target) = &target_cstr {
                args.extend_from_slice(&[c_str!("-target"), target.as_ptr()]);
            }

            let filename = c_str!("header.h");
            let mut sources = [CXUnsavedFile {
                Filename: filename,
                Contents: source.as_ptr() as *const c_char,
                Length: source.len() as c_ulong,
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
                return Err("error building translation unit".into());
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
    EnumDecl,
    EnumConstantDecl,
    VarDecl,
    StructDecl,
    UnionDecl,
    ClassDecl,
    FieldDecl,
    CxxMethod,
    CxxBaseSpecifier,
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
            CXCursor_EnumDecl => CursorKind::EnumDecl,
            CXCursor_EnumConstantDecl => CursorKind::EnumConstantDecl,
            CXCursor_VarDecl => CursorKind::VarDecl,
            CXCursor_StructDecl => CursorKind::StructDecl,
            CXCursor_UnionDecl => CursorKind::UnionDecl,
            CXCursor_ClassDecl => CursorKind::ClassDecl,
            CXCursor_FieldDecl => CursorKind::FieldDecl,
            CXCursor_CXXMethod => CursorKind::CxxMethod,
            CXCursor_CXXBaseSpecifier => CursorKind::CxxBaseSpecifier,
            _ => CursorKind::Other,
        }
    }

    pub fn name(&self) -> StringRef<'a> {
        unsafe { StringRef::from_raw(clang_getCursorSpelling(self.cursor)) }
    }

    pub fn is_anonymous(&self) -> bool {
        unsafe { clang_Cursor_isAnonymous(self.cursor) != 0 }
    }

    pub fn location(&self) -> Location<'a> {
        unsafe { Location::from_raw(clang_getCursorLocation(self.cursor)) }
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

    pub fn enum_integer_type(&self) -> Option<Type<'a>> {
        let type_ = unsafe { clang_getEnumDeclIntegerType(self.cursor) };
        if type_.kind == CXType_Invalid {
            None
        } else {
            Some(unsafe { Type::from_raw(type_) })
        }
    }

    pub fn enum_constant_value(&self) -> Option<c_longlong> {
        if self.kind() == CursorKind::EnumConstantDecl {
            unsafe { Some(clang_getEnumConstantDeclValue(self.cursor)) }
        } else {
            None
        }
    }

    pub fn enum_constant_value_unsigned(&self) -> Option<c_ulonglong> {
        if self.kind() == CursorKind::EnumConstantDecl {
            unsafe { Some(clang_getEnumConstantDeclUnsignedValue(self.cursor)) }
        } else {
            None
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

    pub fn evaluate(&self) -> EvalResult<'a> {
        unsafe { EvalResult::from_raw(clang_Cursor_Evaluate(self.cursor)) }
    }

    pub fn tokens(&self) -> Tokens<'a> {
        unsafe {
            let unit = clang_Cursor_getTranslationUnit(self.cursor);

            let extent = clang_getCursorExtent(self.cursor);
            let start = Location::from_raw(clang_getRangeStart(extent)).file_location();
            let end = Location::from_raw(clang_getRangeEnd(extent)).file_location();

            let physical_start = clang_getLocationForOffset(unit, start.file, start.offset);
            let physical_end = clang_getLocationForOffset(unit, end.file, end.offset);
            let physical_extent = clang_getRange(physical_start, physical_end);

            let mut ptr = NonNull::dangling().as_ptr();
            let mut len = 0;
            clang_tokenize(unit, physical_extent, &mut ptr, &mut len);

            Tokens::from_raw(unit, ptr, len as usize)
        }
    }

    pub fn visit_children<F, E>(&self, mut callback: F) -> Result<(), E>
    where
        F: FnMut(&Cursor) -> Result<(), E>,
    {
        extern "C" fn visitor<E>(
            cursor: CXCursor,
            _parent: CXCursor,
            client_data: CXClientData,
        ) -> CXChildVisitResult {
            let data_ptr = client_data as *mut Data<E>;

            // If a re-entrant call to visit_children panicked, continue unwinding
            let data = unsafe { &*data_ptr };
            if data.panic.is_some() {
                return CXChildVisit_Break;
            }

            let result = catch_unwind(AssertUnwindSafe(|| unsafe {
                let data = &mut *data_ptr;
                (data.callback)(&Cursor::from_raw(cursor))
            }));

            match result {
                Ok(res) => match res {
                    Ok(()) => CXChildVisit_Continue,
                    Err(err) => {
                        let data = unsafe { &mut *data_ptr };
                        data.result = Err(err);
                        CXChildVisit_Break
                    }
                },
                Err(panic) => {
                    let data = unsafe { &mut *data_ptr };
                    data.panic = Some(panic);

                    CXChildVisit_Break
                }
            }
        }

        struct Data<'c, E> {
            callback: &'c mut dyn FnMut(&Cursor) -> Result<(), E>,
            result: Result<(), E>,
            panic: Option<Box<dyn Any + Send + 'static>>,
        }
        let mut data = Data {
            callback: &mut callback,
            result: Ok(()),
            panic: None,
        };

        unsafe {
            clang_visitChildren(
                self.cursor,
                visitor::<E>,
                &mut data as *mut Data<E> as *mut c_void,
            );
        }

        if let Some(panic) = data.panic {
            resume_unwind(panic);
        }

        data.result
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
    Elaborated,
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
            CXType_Elaborated => TypeKind::Elaborated,
            _ => TypeKind::Other,
        }
    }

    pub fn is_const(&self) -> bool {
        unsafe { clang_isConstQualifiedType(self.type_) != 0 }
    }

    pub fn size(&self) -> usize {
        unsafe { clang_Type_getSizeOf(self.type_) as usize }
    }

    #[allow(unused)]
    pub fn name(&self) -> StringRef<'a> {
        unsafe { StringRef::from_raw(clang_getTypeSpelling(self.type_)) }
    }

    pub fn declaration(&self) -> Cursor<'a> {
        unsafe { Cursor::from_raw(clang_getTypeDeclaration(self.type_)) }
    }

    pub fn canonical_type(&self) -> Type<'a> {
        unsafe { Type::from_raw(clang_getCanonicalType(self.type_)) }
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

    pub fn named_type(&self) -> Option<Type<'a>> {
        let named_type = unsafe { clang_Type_getNamedType(self.type_) };
        if named_type.kind == CXType_Invalid {
            None
        } else {
            Some(unsafe { Type::from_raw(named_type) })
        }
    }
}

pub struct Location<'a> {
    location: CXSourceLocation,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Location<'a> {
    unsafe fn from_raw(location: CXSourceLocation) -> Location<'a> {
        Location {
            location,
            _marker: PhantomData,
        }
    }

    pub fn file_location(&self) -> FileLocation<'a> {
        let mut file = ptr::null_mut();
        let mut line = 0;
        let mut column = 0;
        let mut offset = 0;
        unsafe {
            clang_getFileLocation(
                self.location,
                &mut file,
                &mut line,
                &mut column,
                &mut offset,
            );
        }

        FileLocation {
            file,
            line,
            column,
            offset,
            _marker: PhantomData,
        }
    }
}

impl<'a> Display for Location<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let file_location = self.file_location();

        if let Some(filename) = file_location.file_name() {
            write!(
                f,
                "{}:{}:{}",
                filename.to_str().unwrap(),
                file_location.line(),
                file_location.column()
            )?;
        } else {
            write!(f, "<unknown location>")?;
        }

        Ok(())
    }
}

pub struct FileLocation<'a> {
    file: CXFile,
    line: c_uint,
    column: c_uint,
    offset: c_uint,
    _marker: PhantomData<&'a ()>,
}

impl<'a> FileLocation<'a> {
    pub fn file_name(&self) -> Option<StringRef<'a>> {
        if self.file.is_null() {
            return None;
        }

        unsafe { Some(StringRef::from_raw(clang_getFileName(self.file))) }
    }

    pub fn line(&self) -> c_uint {
        self.line
    }

    pub fn column(&self) -> c_uint {
        self.column
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

pub enum EvalResultKind {
    Int,
    Float,
    StrLiteral,
    Other,
}

pub struct EvalResult<'a> {
    result: CXEvalResult,
    _marker: PhantomData<&'a ()>,
}

impl<'a> EvalResult<'a> {
    unsafe fn from_raw(result: CXEvalResult) -> EvalResult<'a> {
        EvalResult {
            result,
            _marker: PhantomData,
        }
    }

    pub fn kind(&self) -> EvalResultKind {
        let kind = unsafe { clang_EvalResult_getKind(self.result) };

        #[allow(non_upper_case_globals)]
        match kind {
            CXEval_Int => EvalResultKind::Int,
            CXEval_Float => EvalResultKind::Float,
            CXEval_StrLiteral => EvalResultKind::StrLiteral,
            _ => EvalResultKind::Other,
        }
    }

    pub fn is_unsigned_int(&self) -> bool {
        unsafe { clang_EvalResult_isUnsignedInt(self.result) != 0 }
    }

    pub fn as_unsigned(&self) -> c_ulonglong {
        unsafe { clang_EvalResult_getAsUnsigned(self.result) }
    }

    pub fn as_long_long(&self) -> c_longlong {
        unsafe { clang_EvalResult_getAsLongLong(self.result) }
    }

    pub fn as_double(&self) -> f64 {
        unsafe { clang_EvalResult_getAsDouble(self.result) }
    }

    pub fn as_str(&self) -> Option<&CStr> {
        unsafe {
            let ptr = clang_EvalResult_getAsStr(self.result);
            if !ptr.is_null() {
                Some(CStr::from_ptr(ptr))
            } else {
                None
            }
        }
    }
}

impl<'a> Drop for EvalResult<'a> {
    fn drop(&mut self) {
        unsafe {
            clang_EvalResult_dispose(self.result);
        }
    }
}

pub struct Tokens<'a> {
    unit: CXTranslationUnit,
    ptr: *mut CXToken,
    len: usize,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Tokens<'a> {
    unsafe fn from_raw(unit: CXTranslationUnit, ptr: *mut CXToken, len: usize) -> Tokens<'a> {
        Tokens {
            unit,
            ptr,
            len,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, index: usize) -> Option<Token<'a>> {
        unsafe {
            let slice = slice::from_raw_parts(self.ptr, self.len as usize);
            slice.get(index).map(|t| Token::from_raw(self.unit, *t))
        }
    }
}

impl<'a> Drop for Tokens<'a> {
    fn drop(&mut self) {
        unsafe {
            clang_disposeTokens(self.unit, self.ptr, self.len.try_into().unwrap());
        }
    }
}

pub struct Token<'a> {
    unit: CXTranslationUnit,
    token: CXToken,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Token<'a> {
    unsafe fn from_raw(unit: CXTranslationUnit, token: CXToken) -> Token<'a> {
        Token {
            unit,
            token,
            _marker: PhantomData,
        }
    }

    pub fn spelling(&self) -> StringRef<'a> {
        unsafe { StringRef::from_raw(clang_getTokenSpelling(self.unit, self.token)) }
    }
}

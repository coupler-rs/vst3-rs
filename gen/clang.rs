use std::any::Any;
use std::ffi::{c_char, c_int, c_void, CStr, CString};
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

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CursorKind {
    Namespace,
    TypedefDecl,
    ClassDecl,
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
            CXCursor_ClassDecl => CursorKind::ClassDecl,
            _ => CursorKind::Other,
        }
    }

    pub fn name(&self) -> StringRef {
        StringRef {
            string: unsafe { clang_getCursorSpelling(self.cursor) },
            _marker: PhantomData,
        }
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

pub struct StringRef<'a> {
    string: CXString,
    _marker: PhantomData<&'a ()>,
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

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::mem::MaybeUninit;

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

    pub fn visit<F>(&self, mut callback: F)
    where
        F: FnMut(&str),
    {
        extern "C" fn visitor(
            cursor: CXCursor,
            _parent: CXCursor,
            client_data: CXClientData,
        ) -> CXChildVisitResult {
            unsafe {
                let data = &mut *(client_data as *mut Data);

                let kind = clang_getCursorKind(cursor);

                let location = clang_getCursorLocation(cursor);
                if clang_Location_isInSystemHeader(location) == 0 {
                    let is_definition =
                        clang_equalCursors(cursor, clang_getCursorDefinition(cursor)) != 0;
                    if kind == CXCursor_ClassDecl && is_definition {
                        let name = clang_getCursorSpelling(cursor);
                        let name_str = clang_getCString(name);

                        (data.callback)(CStr::from_ptr(name_str).to_str().unwrap());

                        clang_disposeString(name);
                    }

                    if kind == CXCursor_Namespace {
                        clang_visitChildren(cursor, visitor, client_data);
                    }
                }

                CXChildVisit_Continue
            }
        }

        struct Data<'c> {
            callback: &'c mut dyn FnMut(&str),
        }
        let mut data = Data {
            callback: &mut callback,
        };

        unsafe {
            let cursor = clang_getTranslationUnitCursor(self.unit);
            clang_visitChildren(cursor, visitor, &mut data as *mut Data as *mut c_void);
        }
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

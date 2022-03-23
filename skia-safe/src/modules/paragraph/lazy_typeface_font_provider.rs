use crate::{
    interop::{self, AsStr},
    prelude::*,
    FontMgr, FontStyleSet, Typeface,
};
use skia_bindings as sb;
use std::{
    fmt,
    mem::transmute,
    ops::{Deref, DerefMut},
    ptr,
};

pub type LazyTypefaceFontProvider = RCHandle<sb::skia_textlayout_LazyTypefaceFontProvider>;

impl NativeRefCountedBase for sb::skia_textlayout_LazyTypefaceFontProvider {
    type Base = sb::SkRefCntBase;
}

impl Deref for LazyTypefaceFontProvider {
    type Target = FontMgr;
    fn deref(&self) -> &Self::Target {
        unsafe { transmute_ref(self) }
    }
}

impl DerefMut for LazyTypefaceFontProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute_ref_mut(self) }
    }
}

impl Default for LazyTypefaceFontProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl From<LazyTypefaceFontProvider> for FontMgr {
    fn from(provider: LazyTypefaceFontProvider) -> Self {
        unsafe { transmute(provider) }
    }
}

impl fmt::Debug for LazyTypefaceFontProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LazyTypefaceFontProvider")
            .field("base", self as &FontMgr)
            .finish()
    }
}

impl LazyTypefaceFontProvider {
    pub fn new() -> Self {
        Self::from_ptr(unsafe { sb::C_LazyTypefaceFontProvider_new() }).unwrap()
    }

    pub fn register_typeface(
        &mut self,
        font_file_path: impl AsRef<str>,
        alias: impl AsRef<str>,
    ) -> usize {
        unsafe {
            let font_file_path = interop::String::from_str(font_file_path.as_ref());
            let alias = interop::String::from_str(alias.as_ref());
            sb::C_LazyTypefaceFontProvider_registerTypeface(
                self.native_mut(),
                font_file_path.native(),
                alias.native(),
            )
        }
    }
}

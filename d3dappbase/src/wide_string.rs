use std;
use std::ffi::OsStr;
use std::os::windows::prelude::*;
use winapi::WCHAR;

pub struct WideString {
    chars: Vec<WCHAR>,
}

impl<'a, T: ?Sized + AsRef<OsStr>> From<&'a T> for WideString {
    fn from(s: &'a T) -> Self {
        WideString {
            chars: s.as_ref().encode_wide()
                .chain(std::iter::once(0))
                .collect()
        }
    }
}

impl WideString {
    pub fn as_ptr(&self) -> *const WCHAR {
        self.chars.as_ptr()
    }
}

use std;
use std::fmt;
use std::ops::{Deref, DerefMut};
use winapi::{HRESULT, IUnknown};

pub struct SafeUnknown<T>(*mut T);

impl<T> Drop for SafeUnknown<T> {
    fn drop(&mut self) {
        unsafe {
            (&mut *(self.0 as *mut IUnknown)).Release();
        }
    }
}

impl<T> Deref for SafeUnknown<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.0 }
    }
}

impl<T> DerefMut for SafeUnknown<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.0 }
    }
}

impl<T> Clone for SafeUnknown<T> {
    fn clone(&self) -> SafeUnknown<T> {
        unsafe {
            (&mut *(self.0 as *mut IUnknown)).AddRef();
        }

        SafeUnknown(self.0)
    }
}

impl<T> SafeUnknown<T> {
    pub unsafe fn from_ptr(ptr: *mut T) -> SafeUnknown<T> {
        if ptr.is_null() { panic!("ptr is null") }
        SafeUnknown(ptr)
    }

    pub fn as_ptr(&self) -> *mut T {
        self.0
    }

    pub unsafe fn borrow_mut(&self) -> &mut T {
        &mut *self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ComError(pub HRESULT);

impl fmt::Display for ComError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "HRESULT 0x{:8X}", self.0)
    }
}

impl std::error::Error for ComError {
    fn description(&self) -> &str {
        "COM error"
    }
}

pub trait HResultExt {
    fn to_result(self) -> Result<(), ComError>;
}

impl HResultExt for HRESULT {
    fn to_result(self) -> Result<(), ComError> {
        if self < 0 { Err(ComError(self)) }
        else { Ok(()) }
    }
}

/*
#[macro_export]
macro_rules! trycom {
    ($e:expr) => ({
        let result = $e;
        if result < 0 {
            return Err($crate::com_support::ComError(result));
        }
    })
}
*/

use std::ops::{Deref, DerefMut};
use winapi::IUnknown;

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

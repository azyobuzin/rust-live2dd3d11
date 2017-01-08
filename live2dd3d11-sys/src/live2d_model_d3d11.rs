use super::*;

use std;
use std::ffi::CStr;
use winapi::{self, c_char, c_float, c_int};

extern {
    fn Live2DModelD3D11_setGraphicsContext(device: *mut winapi::ID3D11Device, context: *mut winapi::ID3D11DeviceContext);
    fn Live2DModelD3D11_deviceLostCommon();
    fn Live2DModelD3D11_loadModelFromFile(filepath: *const c_char) -> LDObjectPtr;
    fn Live2DModelD3D11_loadModelFromBuffer(buf: *const c_void, bufSize: c_int) -> LDObjectPtr;
    fn Live2DModelD3D11_setTexture(p: LDObjectPtr, textureNo: c_int, texture: *mut winapi::ID3D11ShaderResourceView);
    fn Live2DModelD3D11_deleteTextures(p: LDObjectPtr);
    fn Live2DModelD3D11_setMatrix(p: LDObjectPtr, matrix: *mut c_float);
}

pub struct Live2DModelD3D11 {
    ptr: LDObjectPtr,
}

impl Drop for Live2DModelD3D11 {
    fn drop(&mut self) {
        unsafe { super::deleteLDObject(self.ptr) }
    }
}

impl LDObject for Live2DModelD3D11 {
    fn get_ptr(&self) -> LDObjectPtr {
        self.ptr
    }
}

impl Live2DModelD3D11 {
    pub unsafe fn from_ptr(ptr: LDObjectPtr) -> Live2DModelD3D11 {
        Live2DModelD3D11 { ptr: ptr }
    }

    pub unsafe fn setGraphicsContext(device: *mut winapi::ID3D11Device, context: *mut winapi::ID3D11DeviceContext) {
        Live2DModelD3D11_setGraphicsContext(device, context);
    }

    pub unsafe fn deviceLostCommon() {
        Live2DModelD3D11_deviceLostCommon();
    }

    pub fn loadModelFromFile(filepath: &CStr) -> Result<Live2DModelD3D11, ()> {
        let ptr = unsafe { Live2DModelD3D11_loadModelFromFile(filepath.as_ptr()) };

        if ptr.is_null() {
            Err(())
        } else {
            Ok(unsafe { Live2DModelD3D11::from_ptr(ptr) })
        }
    }

    pub fn loadModelFromBuffer(buf: &[u8]) -> Result<Live2DModelD3D11, ()> {
        let ptr = unsafe {
            Live2DModelD3D11_loadModelFromBuffer(
                buf.as_ptr() as *const c_void,
                buf.len() as c_int
            )
        };

        if ptr.is_null() {
            Err(())
        } else {
            Ok(unsafe { Live2DModelD3D11::from_ptr(ptr) })
        }
    }

    pub unsafe fn setTexture(&mut self, textureNo: c_int, texture: *mut winapi::ID3D11ShaderResourceView) {
        Live2DModelD3D11_setTexture(self.ptr, textureNo, texture)
    }

    pub fn deleteTextures(&mut self) {
        unsafe { Live2DModelD3D11_deleteTextures(self.ptr) }
    }

    pub unsafe fn setMatrix(&mut self, matrix: *mut c_float) {
        Live2DModelD3D11_setMatrix(self.ptr, matrix)
    }
}

impl std::ops::Deref for Live2DModelD3D11 {
    type Target = ALive2DModel;

    fn deref(&self) -> &ALive2DModel {
        unsafe { std::mem::transmute(self) }
    }
}

impl std::ops::DerefMut for Live2DModelD3D11 {
    fn deref_mut(&mut self) -> &mut ALive2DModel {
        unsafe { std::mem::transmute(self) }
    }
}

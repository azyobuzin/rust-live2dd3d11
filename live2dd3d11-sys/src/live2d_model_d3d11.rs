use super::*;

use std::ffi::CStr;
use winapi::{self, c_char, c_float, c_int};

extern {
    pub fn Live2DModelD3D11_setGraphicsContext(device: *mut winapi::ID3D11Device, context: *mut winapi::ID3D11DeviceContext);
    pub fn Live2DModelD3D11_deviceLostCommon();
    fn Live2DModelD3D11_loadModelFromFile(filepath: *const c_char) -> LDObjectPtr;
    fn Live2DModelD3D11_loadModelFromBuffer(buf: *const c_void, bufSize: c_int) -> LDObjectPtr;
    fn Live2DModelD3D11_setTexture(p: LDObjectPtr, textureNo: c_int, texture: *mut winapi::ID3D11ShaderResourceView);
    fn Live2DModelD3D11_deleteTextures(p: LDObjectPtr);
    fn Live2DModelD3D11_setMatrix(p: LDObjectPtr, matrix: *mut c_float);
}

pub trait Live2DModelD3D11: LDObject {
    unsafe fn setTexture(&mut self, textureNo: c_int, texture: *mut winapi::ID3D11ShaderResourceView) {
        Live2DModelD3D11_setTexture(self.get_ptr(), textureNo, texture)
    }

    unsafe fn deleteTextures(&mut self) {
        Live2DModelD3D11_deleteTextures(self.get_ptr())
    }

    unsafe fn setMatrix(&mut self, matrix: *mut c_float) {
        Live2DModelD3D11_setMatrix(self.get_ptr(), matrix)
    }
}

pub struct Live2DModelD3D11Instance {
    ptr: LDObjectPtr,
}

impl LDObject for Live2DModelD3D11Instance {
    fn get_ptr(&self) -> LDObjectPtr {
        self.ptr
    }
}

impl Drop for Live2DModelD3D11Instance {
    fn drop(&mut self) {
        unsafe { super::deleteLDObject(self.ptr) }
    }
}

impl ALive2DModel for Live2DModelD3D11Instance { }

impl Live2DModelD3D11 for Live2DModelD3D11Instance { }

impl Live2DModelD3D11Instance {
    pub unsafe fn from_ptr(ptr: LDObjectPtr) -> Live2DModelD3D11Instance {
        Live2DModelD3D11Instance { ptr: ptr }
    }

    pub unsafe fn loadModelFromFile(filepath: &CStr) -> Result<Live2DModelD3D11Instance, ()> {
        let ptr = Live2DModelD3D11_loadModelFromFile(filepath.as_ptr());

        if ptr.is_null() {
            Err(())
        } else {
            Ok(Live2DModelD3D11Instance::from_ptr(ptr))
        }
    }

    pub unsafe fn loadModelFromBuffer(buf: &[u8]) -> Result<Live2DModelD3D11Instance, ()> {
        let ptr = Live2DModelD3D11_loadModelFromBuffer(
            buf.as_ptr() as *const c_void,
            buf.len() as c_int
        );

        if ptr.is_null() {
            Err(())
        } else {
            Ok(Live2DModelD3D11Instance::from_ptr(ptr))
        }
    }
}

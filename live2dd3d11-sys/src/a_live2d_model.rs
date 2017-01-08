use super::*;

use std::ffi::CStr;
use winapi::{c_char, c_float};

extern {
    fn ALive2DModel_getParamFloat(p: LDObjectPtr, paramID: *const c_char) -> c_float;
    fn ALive2DModel_setParamFloat(p: LDObjectPtr, paramID: *const c_char, value: c_float, weight: c_float);
    fn ALive2DModel_update(p: LDObjectPtr);
    fn ALive2DModel_draw(p: LDObjectPtr);
    fn ALive2DModel_getCanvasWidth(p: LDObjectPtr) -> c_float;
    fn ALive2DModel_getCanvasHeight(p: LDObjectPtr) -> c_float;
}

pub struct ALive2DModel {
    ptr: LDObjectPtr,
}

impl Drop for ALive2DModel {
    fn drop(&mut self) {
        unsafe { super::deleteLDObject(self.ptr) }
    }
}

impl LDObject for ALive2DModel {
    fn get_ptr(&self) -> LDObjectPtr {
        self.ptr
    }
}

impl ALive2DModel {
    pub fn getParamFloat(&self, paramID: &CStr) -> c_float {
        unsafe { ALive2DModel_getParamFloat(self.get_ptr(), paramID.as_ptr()) }
    }

    pub fn setParamFloatWithWeight(&mut self, paramID: &CStr, value: c_float, weight: c_float) {
        unsafe { ALive2DModel_setParamFloat(self.get_ptr(), paramID.as_ptr(), value, weight) }
    }

    pub fn setParamFloat(&mut self, paramID: &CStr, value: c_float) {
        self.setParamFloatWithWeight(paramID, value, 1.0)
    }

    pub fn update(&mut self) {
        unsafe { ALive2DModel_update(self.get_ptr()) }
    }

    pub fn draw(&mut self) {
        unsafe { ALive2DModel_draw(self.get_ptr()) }
    }

    pub fn getCanvasWidth(&self) -> c_float {
        unsafe { ALive2DModel_getCanvasWidth(self.get_ptr()) }
    }

    pub fn getCanvasHeight(&self) -> c_float {
        unsafe { ALive2DModel_getCanvasHeight(self.get_ptr()) }
    }
}

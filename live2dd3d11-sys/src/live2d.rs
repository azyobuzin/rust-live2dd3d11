use super::*;
use super::w_bool;

use std::ffi::CStr;
use winapi::{c_char, c_int};

extern { fn Live2D_getClippingMaskBufferSize() -> c_int; }
pub fn getClippingMaskBufferSize() -> c_int {
    unsafe { Live2D_getClippingMaskBufferSize() }
}

extern { fn Live2D_setClippingMaskBufferSize(size: c_int); }
pub unsafe fn setClippingMaskBufferSize(size: c_int) {
    Live2D_setClippingMaskBufferSize(size)
}

extern { fn Live2D_init(); }
pub unsafe fn init() {
    Live2D_init()
}

extern { fn Live2D_dispose(); }
pub unsafe fn dispose() {
    Live2D_dispose()
}

extern { fn Live2D_getVersionStr() -> *const c_char; }
pub fn getVersionStr<'a>() -> &'a CStr {
    unsafe { CStr::from_ptr(Live2D_getVersionStr()) }
}

extern { fn Live2D_getVersionNo() -> l2d_uint32; }
pub fn getVersionNo() -> l2d_uint32 {
    unsafe { Live2D_getVersionNo() }
}

extern { fn Live2D_getBuildOption_RANGE_CHECK_POINT() -> w_bool; }
pub fn getBuildOption_RANGE_CHECK_POINT() -> bool {
    unsafe { Live2D_getBuildOption_RANGE_CHECK_POINT() != 0 }
}

extern { fn Live2D_getBuildOption_AVATAR_OPTION_A() -> w_bool; }
pub fn getBuildOption_AVATAR_OPTION_A() -> bool {
    unsafe { Live2D_getBuildOption_AVATAR_OPTION_A() != 0 }
}

extern { fn Live2D_setVertexDoubleBufferEnabled(enabled: w_bool); }
pub unsafe fn setVertexDoubleBufferEnabled(enabled: bool) {
    let x = if enabled { 1 } else { 0 };
    Live2D_setVertexDoubleBufferEnabled(x)
}

extern { fn Live2D_isVertexDoubleBufferEnabled() -> w_bool; }
pub fn isVertexDoubleBufferEnabled() -> bool {
    unsafe { Live2D_isVertexDoubleBufferEnabled() != 0 }
}

extern { fn Live2D_setError(errorNo: l2d_uint32); }
pub unsafe fn setError(errorNo: l2d_uint32) {
    Live2D_setError(errorNo)
}

extern { fn Live2D_getError() -> l2d_uint32; }
pub fn getError() -> l2d_uint32 {
    unsafe { Live2D_getError() }
}

#![allow(non_snake_case, non_camel_case_types)]

extern crate winapi;

use winapi::{c_float, c_int, c_longlong, c_schar, c_short, c_uchar, c_uint, c_ulonglong, c_ushort, c_void, size_t};

// bool representaion in the wrapper
type w_bool = c_int;

pub type l2d_uvmapf = c_float;
pub type l2d_pointf = c_float;
pub type l2d_paramf = c_float;
pub type l2d_index = c_ushort;
pub type l2d_order = c_short;
pub type l2d_float = c_float;
pub type l2d_size_t = size_t;
pub type l2d_int8 = c_schar;
pub type l2d_uint8 = c_uchar;
pub type l2d_int16 = c_short;
pub type l2d_uint16 = c_ushort;
pub type l2d_int32 = c_int;
pub type l2d_uint32 = c_uint;
pub type l2d_int64 = c_longlong;
pub type l2d_uint64 = c_ulonglong;

pub type LDObjectPtr = *mut c_void;

extern {
    fn deleteLDObject(obj: LDObjectPtr);
}

pub trait LDObject {
    fn get_ptr(&self) -> LDObjectPtr;
}

mod live2d;
pub use self::live2d::*;

mod a_live2d_model;
pub use self::a_live2d_model::*;

mod live2d_model_d3d11;
pub use self::live2d_model_d3d11::*;

pub mod ut_system;

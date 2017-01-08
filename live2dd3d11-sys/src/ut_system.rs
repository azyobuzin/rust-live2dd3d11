use super::*;
use super::w_bool;

extern { fn UtSystem_isBigEndian() -> w_bool; }
pub fn isBigEndian() -> bool {
    unsafe { UtSystem_isBigEndian() != 0 }
}

extern { fn UtSystem_getTimeMSec() -> l2d_int64; }
pub fn getTimeMSec() -> l2d_int64 {
    unsafe { UtSystem_getTimeMSec() }
}

extern { fn UtSystem_getUserTimeMSec() -> l2d_int64; }
pub fn getUserTimeMSec() -> l2d_int64 {
    unsafe { UtSystem_getUserTimeMSec() }
}

extern { fn UtSystem_setUserTimeMSec(t: l2d_int64); }
pub unsafe fn setUserTimeMSec(t: l2d_int64) {
    UtSystem_setUserTimeMSec(t)
}

extern { fn UtSystem_updateUserTimeMSec() -> l2d_int64; }
pub unsafe fn updateUserTimeMSec() -> l2d_int64 {
    UtSystem_updateUserTimeMSec()
}

extern { fn UtSystem_resetUserTimeMSec(); }
pub unsafe fn resetUserTimeMSec() {
    UtSystem_resetUserTimeMSec()
}

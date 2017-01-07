use winapi::HWND;

#[derive(Debug)]
pub struct WindowHandle {
    hwnd: HWND,
    destroyed: bool,
}

impl !Send for WindowHandle { }
impl !Sync for WindowHandle { }

impl WindowHandle {
    pub fn from_hwnd(hwnd: HWND) -> WindowHandle {
        WindowHandle { hwnd: hwnd, destroyed: false }
    }

    pub fn get_hwnd(&self) -> HWND {
        self.hwnd
    }

    pub fn destroy(&mut self) {
        if !self.destroyed {
            self.destroyed = true;
            unsafe {
                ::user32::DestroyWindow(self.hwnd);
            }
        }
    }

    pub unsafe fn mark_as_destroyed(&mut self) {
        self.destroyed = true;
    }
}

impl Drop for WindowHandle {
    fn drop(&mut self) {
        self.destroy();
    }
}

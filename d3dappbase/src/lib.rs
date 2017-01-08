#![feature(optin_builtin_traits)]

extern crate d3d11;
extern crate user32;
extern crate winapi;

mod d3d_init;
mod safe_unknown;
mod safe_window_handle;
mod wide_string;

pub use safe_unknown::SafeUnknown;

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::mem;
use std::ptr::{null, null_mut};
use std::rc::*;
use d3d_init::*;
use safe_window_handle::*;
use wide_string::*;

pub trait Renderer {
    fn render(&mut self, window: &D3dAppWindow);
}

#[derive(Debug, Clone, Copy)]
pub struct WindowConfig<'a> {
    pub class_name: &'a OsStr,
    pub title: &'a OsStr,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone)]
pub struct Direct3DConfig<'a> {
    pub format: winapi::DXGI_FORMAT,
    pub feature_levels: &'a [winapi::D3D_FEATURE_LEVEL],
    pub refresh_rate_numerator: u32,
    pub refresh_rate_denominator: u32,
}

thread_local! {
    // スレッドごとの D3dAppInner
    static TL_APP_PTR: RefCell<Weak<RefCell<D3dAppInner>>> = Default::default();
}

type HandlesTable = HashMap<winapi::HWND, D3dAppWindow>;

struct D3dAppInner {
    windows: HandlesTable,
}

impl D3dAppInner {
    fn is_managed_window(&self, handle: winapi::HWND) -> bool {
        self.windows.contains_key(&handle)
    }
}

pub struct D3dApp {
    inner: Rc<RefCell<D3dAppInner>>,
}

impl D3dApp {
    pub fn new() -> D3dApp {
        if TL_APP_PTR.with(|x| x.borrow().upgrade().is_some()) {
            panic!("a D3dApp has already been created in the current thread");
        }

        let inner = Rc::new(RefCell::new(D3dAppInner { windows: HashMap::new() }));
        TL_APP_PTR.with(|x| *x.borrow_mut() = Rc::downgrade(&inner));
        D3dApp { inner: inner }
    }

    pub fn create_window(&mut self, window_config: WindowConfig, d3d_config: Direct3DConfig, renderer: Box<Renderer>)
        -> Result<(), ()>
    {
        let handle = create_window_core(&window_config)?;
        let device = initialize_direct3d(&d3d_config, &window_config, handle.get_hwnd())?;

        let insert_result = self.inner.borrow_mut().windows.insert(
            handle.get_hwnd(),
            D3dAppWindow {
                handle: handle,
                d3d_device_resources: device,
                renderer: renderer.into(),
            }
        );

        if insert_result.is_some() { panic!("duplicated window handle") }

        Ok(())
    }

    pub fn is_managed_window(&self, handle: winapi::HWND) -> bool {
        self.inner.borrow().is_managed_window(handle)
    }

    pub fn main_loop(self) -> i32 {
        unsafe {
            let mut msg: winapi::MSG = std::mem::uninitialized();

            loop {
                if user32::PeekMessageW(&mut msg, null_mut(), 0, 0, winapi::PM_NOREMOVE) != 0 {
                    if user32::GetMessageW(&mut msg, null_mut(), 0, 0) == 0 { return msg.wParam as i32; }

                    user32::TranslateMessage(&mut msg);
                    user32::DispatchMessageW(&mut msg);
                } else {
                    for w in self.inner.borrow().windows.values() {
                        w.render()
                    }
                }
            }
        }
    }
}

pub struct D3dAppWindow {
    pub handle: WindowHandle,
    pub d3d_device_resources: D3dDeviceResources,
    renderer: RefCell<Box<Renderer>>,
}

impl D3dAppWindow {
    fn render(&self) {
        self.renderer.borrow_mut().render(self);

        unsafe {
            self.d3d_device_resources.swap_chain
                .borrow_mut()
                .Present(0, 0);
        }
    }
}

pub struct D3dDeviceResources {
    pub swap_chain: SafeUnknown<winapi::IDXGISwapChain>,
    pub device: SafeUnknown<winapi::ID3D11Device>,
    pub feature_level: winapi::D3D_FEATURE_LEVEL,
    pub immediate_context: SafeUnknown<winapi::ID3D11DeviceContext>,
    pub back_buffer_render_target_view: SafeUnknown<winapi::ID3D11RenderTargetView>,
}

fn create_window_core(config: &WindowConfig) -> Result<WindowHandle, ()> {
    let class_name = WideString::from(config.class_name);
    let title = WideString::from(config.title);

    let wcex = winapi::WNDCLASSEXW {
        cbSize: mem::size_of::<winapi::WNDCLASSEXW>() as winapi::UINT,
        style: winapi::CS_HREDRAW | winapi::CS_VREDRAW,
        lpfnWndProc: Some(wndproc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: null_mut(),
        hIcon: null_mut(),
        hCursor: unsafe { user32::LoadCursorW(null_mut(), winapi::IDC_ARROW) },
        hbrBackground: (winapi::COLOR_WINDOW + 1) as winapi::HBRUSH,
        lpszMenuName: null(),
        lpszClassName: class_name.as_ptr(),
        hIconSm: null_mut(),
    };

    let class_atom = unsafe {
        user32::RegisterClassExW(&wcex)
    };

    // TODO: Error handling
    if class_atom == 0 { return Err(()); }

    let handle = unsafe {
        user32::CreateWindowExW(
            0,
            class_name.as_ptr(),
            title.as_ptr(),
            winapi::WS_OVERLAPPEDWINDOW,
            winapi::CW_USEDEFAULT, 0,
            config.width, config.height,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut()
        )
    };

    // TODO: Error handling
    if handle.is_null() { return Err(()); }

    unsafe {
        user32::ShowWindow(handle, winapi::SW_SHOWDEFAULT);
        user32::UpdateWindow(handle);
    }

    Ok(WindowHandle::from_hwnd(handle))
}

unsafe extern "system" fn wndproc(hwnd: winapi::HWND, message: winapi::UINT, wparam: winapi::WPARAM, lparam: winapi::LPARAM) -> winapi::LRESULT {
    if let Some(cell_app) = TL_APP_PTR.with(|x| x.borrow().upgrade()) {
        match message {
            winapi::WM_DESTROY => {
                let quit = {
                    let mut windows = &mut cell_app.borrow_mut().windows;
                    if let Some(mut x) = windows.remove(&hwnd) {
                        x.handle.mark_as_destroyed();
                    }
                    windows.len() == 0
                };
                if quit {
                    // すべてのウィンドウが破棄されたら終了する
                    user32::PostQuitMessage(0);
                }
            }
            winapi::WM_CLOSE => {
                let managed = cell_app.borrow().is_managed_window(hwnd);
                if managed {
                    user32::DestroyWindow(hwnd);
                }
            }
            _ => { }
        }
    }

    user32::DefWindowProcW(hwnd, message, wparam, lparam)
}

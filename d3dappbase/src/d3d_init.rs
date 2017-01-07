use std;
use std::mem;
use std::ptr::{null, null_mut};
use d3d11;
use winapi;
use self::safe_unknown::SafeUnknown;

mod safe_unknown {
    use std::ops::{Deref, DerefMut};
    use winapi;

    pub struct SafeUnknown<T: DerefMut<Target=winapi::IUnknown>>(*mut T);
    
    impl<T: DerefMut<Target=winapi::IUnknown>> Drop for SafeUnknown<T> {
        fn drop(&mut self) {
            unsafe { self.as_ref().Release(); }
        }
    }

    impl<T: DerefMut<Target=winapi::IUnknown>> Deref for SafeUnknown<T> {
        type Target = T;

        fn deref(&self) -> &T {
            unsafe { self.0.as_ref().unwrap() }
        }
    }

    impl<T: DerefMut<Target=winapi::IUnknown>> DerefMut for SafeUnknown<T> {
        fn deref_mut(&mut self) -> &mut T {
            unsafe { self.0.as_mut().unwrap() }
        }
    }

    impl<T: DerefMut<Target=winapi::IUnknown>> SafeUnknown<T> {
        pub unsafe fn from_ptr(ptr: *mut T) -> SafeUnknown<T> {
            if ptr.is_null() { panic!("ptr is null") }
            SafeUnknown(ptr)
        }
    }
}

macro_rules! trycom {
    ($e:expr) => ((if ($e) < 0 { return Err(()); }))
}

pub fn initialize_direct3d(config: &super::Direct3DConfig, window_config: &super::WindowConfig, handle: winapi::HWND) -> Result<(), ()> {
    let (swap_chain, device, feature_level, immediate_context) = {
        let swap_chain_desc = winapi::DXGI_SWAP_CHAIN_DESC {
            BufferDesc: winapi::DXGI_MODE_DESC {
                Width: window_config.width as winapi::UINT,
                Height: window_config.height as winapi::UINT,
                RefreshRate: winapi::DXGI_RATIONAL {
                    Numerator: config.refresh_rate_numerator,
                    Denominator: config.refresh_rate_denominator,
                },
                Format: config.format,
                ScanlineOrdering: winapi::DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                Scaling: winapi::DXGI_MODE_SCALING_UNSPECIFIED,
            },
            SampleDesc: winapi::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferUsage: winapi::DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 1,
            OutputWindow: handle,
            Windowed: winapi::TRUE,
            SwapEffect: winapi::DXGI_SWAP_EFFECT_DISCARD,
            Flags: 0,
        };

        unsafe {
            let mut swap_chain: *mut winapi::IDXGISwapChain = mem::uninitialized();
            let mut device: *mut winapi::ID3D11Device = mem::uninitialized();
            let mut feature_level: winapi::D3D_FEATURE_LEVEL = mem::uninitialized();
            let mut immediate_context: *mut winapi::ID3D11DeviceContext = mem::uninitialized();

            trycom!(d3d11::D3D11CreateDeviceAndSwapChain(
                null_mut(),
                winapi::D3D_DRIVER_TYPE_HARDWARE,
                null_mut(),
                0,
                config.feature_levels.as_ptr(),
                config.feature_levels.len() as u32,
                winapi::D3D11_SDK_VERSION,
                &swap_chain_desc as *const winapi::DXGI_SWAP_CHAIN_DESC,
                &mut swap_chain as *mut *mut winapi::IDXGISwapChain,
                &mut device as *mut *mut winapi::ID3D11Device,
                &mut feature_level as *mut winapi::D3D_FEATURE_LEVEL,
                &mut immediate_context as *mut *mut winapi::ID3D11DeviceContext
            ));

            (SafeUnknown::from_ptr(swap_chain), SafeUnknown::from_ptr(device), feature_level, SafeUnknown::from_ptr(immediate_context))
        }
    };

    Ok(()) // TODO
}

use std::mem;
use std::ptr::{null, null_mut};
use d3d11;
use winapi;
use super::*;

/// MIDL_INTERFACE("6f15aaf2-d208-4e89-9ab4-489535d34f9c")
#[allow(non_upper_case_globals)]
const IID_ID3D11Texture2D: winapi::IID = winapi::IID {
    Data1: 0x6f15aaf2,
    Data2: 0xd208,
    Data3: 0x4e89,
    Data4: [0x9a, 0xb4, 0x48, 0x95, 0x35, 0xd3, 0x4f, 0x9c],
};

pub fn initialize_direct3d(config: &super::Direct3DConfig, window_config: &super::WindowConfig, handle: winapi::HWND)
    -> Result<D3dDeviceResources, ()>
{
    unsafe {
        // デバイスとスワップチェーン
        let (mut swap_chain, mut device, feature_level, mut immediate_context) = {
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

            let mut swap_chain: *mut winapi::IDXGISwapChain = null_mut();
            let mut device: *mut winapi::ID3D11Device = null_mut();
            let mut feature_level: winapi::D3D_FEATURE_LEVEL = mem::uninitialized();
            let mut immediate_context: *mut winapi::ID3D11DeviceContext = null_mut();

            trycom!(d3d11::D3D11CreateDeviceAndSwapChain(
                null_mut(),
                winapi::D3D_DRIVER_TYPE_HARDWARE,
                null_mut(),
                0,
                config.feature_levels.as_ptr(),
                config.feature_levels.len() as winapi::UINT,
                winapi::D3D11_SDK_VERSION,
                &swap_chain_desc,
                &mut swap_chain,
                &mut device,
                &mut feature_level,
                &mut immediate_context
            ));

            (
                SafeUnknown::from_ptr(swap_chain),
                SafeUnknown::from_ptr(device),
                feature_level,
                SafeUnknown::from_ptr(immediate_context)
            )
        };

        // レンダーターゲットビュー
        let back_buffer_rtv = {
            let back_buffer_texture = {
                let mut p: *mut winapi::ID3D11Texture2D = null_mut();
                trycom!(swap_chain.GetBuffer(
                    0,
                    &IID_ID3D11Texture2D,
                    (&mut p as *mut *mut winapi::ID3D11Texture2D) as *mut *mut winapi::c_void
                ));
                SafeUnknown::from_ptr(p)
            };
            let mut p: *mut winapi::ID3D11RenderTargetView = null_mut();
            trycom!(device.CreateRenderTargetView(
                back_buffer_texture.as_ptr() as *mut winapi::ID3D11Resource,
                null(),
                &mut p
            ));
            SafeUnknown::from_ptr(p)
        };

        let render_target_views = [back_buffer_rtv.as_ptr()];
        immediate_context.OMSetRenderTargets(
            render_target_views.len() as winapi::UINT,
            render_target_views.as_ptr(),
            null_mut()
        );

        let view_ports = [
            winapi::D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: window_config.width as winapi::FLOAT,
                Height: window_config.height as winapi::FLOAT,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            }
        ];
        immediate_context.RSSetViewports(view_ports.len() as winapi::UINT, view_ports.as_ptr());

        Ok(D3dDeviceResources {
            swap_chain: swap_chain,
            device: device,
            feature_level: feature_level,
            immediate_context: immediate_context,
            back_buffer_render_target_view: back_buffer_rtv,
        })
    }
}

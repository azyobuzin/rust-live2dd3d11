extern crate d3dappbase;
extern crate image;
extern crate live2dd3d11_sys as l2d;
extern crate winapi;
extern crate xmath;

use std::ffi::{CStr, CString};
use std::fmt;
use std::path;
use std::ptr::null_mut;
use d3dappbase::*;
use d3dappbase::com_support::*;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 800;

fn main() {
    let mut app = D3dApp::new();
    app.create_window(
        WindowConfig {
            class_name: "RustLive2DSimple".as_ref(),
            title: "Simple".as_ref(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
        },
        Direct3DConfig {
            format: winapi::DXGI_FORMAT_R8G8B8A8_UNORM,
            feature_levels: &[winapi::D3D_FEATURE_LEVEL_10_0],
            refresh_rate_numerator: 60,
            refresh_rate_denominator: 1,
        },
        Box::new(SimpleRenderer::Uninitialized)
    ).unwrap();
    app.main_loop();
    unsafe { l2d::dispose(); }
}

struct SimpleRendererState {
    live2d_model: l2d::Live2DModelD3D11,
    #[allow(dead_code)]
    textures: Vec<SafeUnknown<winapi::ID3D11ShaderResourceView>>,
}

enum SimpleRenderer {
    Uninitialized,
    Initialized(SimpleRendererState),
    Error,
}

impl Renderer for SimpleRenderer {
    fn render(&mut self, window: &D3dAppWindow) {
        // とりあえず白で消しておく
        unsafe {
            const CLEAR_COLOR: [winapi::c_float; 4] = [1.0, 1.0, 1.0, 1.0];
            window.d3d_device_resources.immediate_context
                .borrow_mut()
                .ClearRenderTargetView(
                    window.d3d_device_resources.back_buffer_render_target_view.as_ptr(),
                    &CLEAR_COLOR
                );
        }

        match *self {
            SimpleRenderer::Initialized(ref mut state) => {
                render_core(state);
                return;
            }
            SimpleRenderer::Error => { return; }
            SimpleRenderer::Uninitialized => { /* 下で初期化 */ }
        }

        match initialize(window) {
            Ok(x) => {
                *self = SimpleRenderer::Initialized(x);
            }
            Err(_) => {
                // エラー起こしたらさっさと終了
                window.close();
                *self = SimpleRenderer::Error;
            }
        }
    }
}

fn initialize(window: &D3dAppWindow) -> Result<SimpleRendererState, ()> {
    let sdk_dir = {
        // ヘッダーディレクトリの 1 個上とかいうクソみたいな判定
        match std::env::var_os("LIVE2DD3D11_INCLUDE_DIR") {
            Some(include_dir) => {
                let mut pb = path::PathBuf::from(include_dir);
                if !pb.pop() {
                    println!("couldn't find the sdk directory");
                    return Err(());
                }
                pb
            },
            None => {
                println!("LIVE2DD3D11_INCLUDE_DIR is not set");
                return Err(());
            }
        }
    };

    unsafe {
        l2d::init();
        l2d::Live2DModelD3D11::setGraphicsContext(
            window.d3d_device_resources.device.as_ptr(),
            window.d3d_device_resources.immediate_context.as_ptr()
        );
    }

    let mut live2d_model = {
        let s = sdk_dir.join("sample/Simple/res/epsilon/Epsilon2.1.moc").to_str()
            .and_then(|x| CString::new(x).ok());
        match s {
            Some(x) => match l2d::Live2DModelD3D11::loadModelFromFile(&x) {
                Ok(x) => x,
                Err(_) => {
                    println!("invalid model");
                    return Err(());
                }
            },
            None => {
                println!("invalid model path");
                return Err(());
            }
        }
    };

    // 読み込みに失敗したかどうかわからないクソAPIなので、サイズチェックしておく
    if live2d_model.getCanvasWidth() == 0.0 || live2d_model.getCanvasHeight() == 0.0 {
        println!("invalid model");
        return Err(());
    }

    let texture_paths = [
        sdk_dir.join("sample/Simple/res/epsilon/Epsilon2.1.2048/texture_00.png")
    ];
    let mut textures = Vec::with_capacity(texture_paths.len());
    for (i, x) in texture_paths.iter().enumerate() {
        let mut device = unsafe { window.d3d_device_resources.device.borrow_mut() };
        match create_texture_view(device, x) {
            Ok(srv) => {
                unsafe {
                    live2d_model.setTexture(i as winapi::c_int, srv.as_ptr());
                }
                textures.push(srv);
            }
            Err(x) => {
                println!("couldn't load the texture: {}", x);
                return Err(());
            }
        }
    }

    // 原作通りの座標変換
    let aspect = (WINDOW_HEIGHT as f32) / (WINDOW_WIDTH as f32);
    let model_width = live2d_model.getCanvasWidth();
    let model_height = live2d_model.getCanvasHeight();
    let ortho = orthographic_lh(model_height, -model_height * aspect, -1.0, 1.0);
    let trans = xmath::Matrix::translation(-model_width / 2.0, -model_height / 2.0, 0.0);
    let mut m: [[f32; 4]; 4] = (trans * ortho).transpose().into();
    unsafe { live2d_model.setMatrix(m.as_mut_ptr() as *mut winapi::c_float); }

    Ok(SimpleRendererState {
        live2d_model: live2d_model,
        textures: textures,
    })
}

fn render_core(state: &mut SimpleRendererState) {
    static PARAM_ANGLE_X: &'static [u8] = b"PARAM_ANGLE_X\0";
    static PARAM_EYE_L_OPEN: &'static [u8] = b"PARAM_EYE_L_OPEN\0";
    static PARAM_EYE_R_OPEN: &'static [u8] = b"PARAM_EYE_R_OPEN\0";

    let t = (l2d::ut_system::getUserTimeMSec() as f64) / 1000.0 * 2.0 * std::f64::consts::PI;
    unsafe {
        state.live2d_model.setParamFloat(
            CStr::from_ptr(PARAM_ANGLE_X.as_ptr() as *const winapi::c_char),
            (30.0 * (t / 3.0).sin()) as winapi::c_float
        );
        state.live2d_model.setParamFloat(
            CStr::from_ptr(PARAM_EYE_L_OPEN.as_ptr() as *const winapi::c_char),
            (1.0 + (t / 3.0).sin()) as winapi::c_float
        );
        state.live2d_model.setParamFloat(
            CStr::from_ptr(PARAM_EYE_R_OPEN.as_ptr() as *const winapi::c_char),
            (1.0 + (t / 3.0).sin()) as winapi::c_float
        );

        state.live2d_model.update();
        state.live2d_model.draw();
    }
}

#[derive(Debug)]
enum CreateTextureViewError {
    LoadImage(image::ImageError),
    DirectX(ComError),
}

impl From<image::ImageError> for CreateTextureViewError {
    fn from(e: image::ImageError) -> CreateTextureViewError {
        CreateTextureViewError::LoadImage(e)
    }
}

impl From<ComError> for CreateTextureViewError {
    fn from(e: ComError) -> CreateTextureViewError {
        CreateTextureViewError::DirectX(e)
    }
}

impl fmt::Display for CreateTextureViewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CreateTextureViewError::LoadImage(ref x) => fmt::Display::fmt(x, f),
            CreateTextureViewError::DirectX(ref x) => fmt::Display::fmt(x, f),
        }
    }
}

fn create_texture_view<P: AsRef<path::Path>>(device: &mut winapi::ID3D11Device, path: P)
    -> Result<SafeUnknown<winapi::ID3D11ShaderResourceView>, CreateTextureViewError>
{
    let img = match image::open(path)? {
        image::DynamicImage::ImageRgba8(x) => x,
        x => x.to_rgba(),
    };

    let texture_desc = winapi::D3D11_TEXTURE2D_DESC {
        Width: img.width(),
        Height: img.height(),
        MipLevels: 1,
        ArraySize: 1,
        Format: winapi::DXGI_FORMAT_R8G8B8A8_UNORM,
        SampleDesc: winapi::DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Usage: winapi::D3D11_USAGE_DEFAULT,
        BindFlags: winapi::D3D11_BIND_SHADER_RESOURCE.0,
        CPUAccessFlags: 0,
        MiscFlags: 0,
    };

    let img_data = img.into_raw();
    let initial_data = winapi::D3D11_SUBRESOURCE_DATA {
        pSysMem: img_data.as_ptr() as *const winapi::c_void,
        SysMemPitch: texture_desc.Width * 4,
        SysMemSlicePitch: 0,
    };

    let texture = unsafe {
        let mut p: *mut winapi::ID3D11Texture2D = null_mut();
        device.CreateTexture2D(
            &texture_desc,
            &initial_data,
            &mut p
        ).to_result()?;
        SafeUnknown::from_ptr(p)
    };

    let srv_desc = winapi::D3D11_SHADER_RESOURCE_VIEW_DESC {
        Format: winapi::DXGI_FORMAT_R8G8B8A8_UNORM,
        ViewDimension: winapi::D3D11_SRV_DIMENSION_TEXTURE2D,
        u: [
            0, // MostDetailedMip
            1, // MipLevels
            0,
            0,
        ],
    };

    Ok(unsafe {
        let mut p: *mut winapi::ID3D11ShaderResourceView = null_mut();
        device.CreateShaderResourceView(
            texture.as_ptr() as *mut winapi::ID3D11Resource,
            &srv_desc,
            &mut p
        ).to_result()?;
        SafeUnknown::from_ptr(p)
    })
}

fn orthographic_lh(view_width: f32, view_height: f32, near_z: f32, far_z: f32) -> xmath::Matrix {
    let f_range = 1.0 / (far_z - near_z);
    xmath::Matrix::new(
        2.0 / view_width, 0.0, 0.0, 0.0,
        0.0, 2.0 / view_height, 0.0, 0.0,
        0.0, 0.0, f_range, 0.0,
        0.0, 0.0, -f_range * near_z, 1.0
    )
}

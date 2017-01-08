#[macro_use] extern crate d3dappbase;
extern crate image;
extern crate live2dd3d11_sys as l2d;
extern crate winapi;
extern crate xmath;

use std::ffi::CString;
use std::path;
use d3dappbase::*;

fn main() {
    let mut app = D3dApp::new();
    app.create_window(
        WindowConfig {
            class_name: "RustLive2DSimple".as_ref(),
            title: "Simple".as_ref(),
            width: 800,
            height: 800,
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
}

struct SimpleRendererState {
    live2d_model: l2d::Live2DModelD3D11,
    textures: Vec<winapi::ID3D11ShaderResourceView>,
}

enum SimpleRenderer {
    Uninitialized,
    Initialized(SimpleRendererState),
    Error,
}

impl Renderer for SimpleRenderer {
    fn render(&mut self, window: &D3dAppWindow) {
        match *self {
            SimpleRenderer::Initialized(ref mut state) => {
                render_core(state, window);
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

impl Drop for SimpleRenderer {
    fn drop(&mut self) {
        unsafe { l2d::dispose(); }
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

    unimplemented!() // TODO: テクスチャつくり
}

fn render_core(state: &mut SimpleRendererState, window: &D3dAppWindow) {
    // TODO
}

fn create_texture_view<P: AsRef<path::Path>>(path: P) -> Result<SafeUnknown<winapi::ID3D11ShaderResourceView>, ()> {
    unimplemented!() // TODO
}

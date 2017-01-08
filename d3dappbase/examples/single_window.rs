extern crate d3dappbase;
extern crate winapi;

use d3dappbase::*;

fn main() {
    let mut app = D3dApp::new();
    app.create_window(
        WindowConfig {
            class_name: "SingleWindow".as_ref(),
            title: "d3dappbase テスト".as_ref(),
            width: 450,
            height: 400,
        },
        Direct3DConfig {
            format: winapi::DXGI_FORMAT_R8G8B8A8_UNORM,
            feature_levels: &[winapi::D3D_FEATURE_LEVEL_10_0],
            refresh_rate_numerator: 60,
            refresh_rate_denominator: 1,
        },
        Box::new(ClearRenderer)
    ).unwrap();

    let exit_code = app.main_loop();
    println!("Exit code: {}", exit_code);
}

struct ClearRenderer;

impl Renderer for ClearRenderer {
    fn render(&mut self, window: &D3dAppWindow) {
        const CORNFLOWER_BLUE: [winapi::FLOAT; 4] = [0.392156899, 0.584313750, 0.929411829, 1.000000000];

        unsafe {
            window.d3d_device_resources.immediate_context
                .borrow_mut()
                .ClearRenderTargetView(
                    window.d3d_device_resources.back_buffer_render_target_view.as_ptr(),
                    &CORNFLOWER_BLUE
                );
        }
    }
}

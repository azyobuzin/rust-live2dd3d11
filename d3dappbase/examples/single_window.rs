extern crate d3dappbase;

fn main() {
    let mut app = d3dappbase::D3dApp::new();
    app.create_window("SingleWindow", "SingleWindow", 450, 400, Box::new(NopRenderer)).unwrap();
    let exit_code = app.main_loop();
    println!("Exit code: {}", exit_code);
}

struct NopRenderer;

impl d3dappbase::Renderer for NopRenderer {
    fn render(&mut self, _: &d3dappbase::D3dAppWindow) { }
}

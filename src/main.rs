slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let platform = Box::new(
        i_slint_backend_winit::Backend::new_with_renderer_by_name(Some("skia-opengl")).unwrap(),
    );
    slint::platform::set_platform(platform).unwrap();
    
    let ui = AppWindow::new()?;

    ui.run()
}

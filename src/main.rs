mod app;
mod camera;
mod input;
mod renderer;
mod uniforms;

use log::info;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

use app::RayMarchingApp;

async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = event_loop.create_window(
        Window::default_attributes()
            .with_title("3D Fractal Ray Marcher")
            .with_inner_size(winit::dpi::LogicalSize::new(1024, 768))
    ).unwrap();

    let mut app = RayMarchingApp::new(&window).await;

    let _ = event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == app.window().id() => {
                if !app.input(event) {
                    match event {
                        event if app.should_exit(event) => control_flow.exit(),
                        WindowEvent::Resized(physical_size) => {
                            app.resize(*physical_size);
                        }
                        WindowEvent::RedrawRequested => {
                            app.update();
                            match app.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost) => app.resize(app.renderer.size),
                                Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                app.window().request_redraw();
            }
            _ => {}
        }
    });
}

fn main() {
    pollster::block_on(run());
}
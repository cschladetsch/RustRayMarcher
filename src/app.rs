use std::time::Instant;
use winit::{
    event::{Event, WindowEvent, KeyEvent, ElementState},
    event_loop::EventLoop,
    window::Window,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::camera::Camera;
use crate::input::InputHandler;
use crate::renderer::Renderer;
use crate::uniforms::Uniforms;

pub struct RayMarchingApp<'a> {
    pub renderer: Renderer<'a>,
    pub uniforms: Uniforms,
    pub camera: Camera,
    pub input_handler: InputHandler,
    pub last_frame_time: Instant,
    pub frame_count: u32,
    pub fps_update_timer: f32,
    pub current_fps: f32,
    pub window: &'a Window,
}

impl<'a> RayMarchingApp<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let renderer = Renderer::new(window).await;
        let camera = Camera::new(renderer.size.width as f32 / renderer.size.height as f32);
        let mut uniforms = Uniforms::new();
        
        let (view_matrix, projection_matrix) = camera.build_view_projection_matrix();
        uniforms.view_matrix = view_matrix.into();
        uniforms.projection_matrix = projection_matrix.into();
        uniforms.camera_pos = camera.position.into();

        Self {
            renderer,
            uniforms,
            camera,
            input_handler: InputHandler::new(),
            last_frame_time: Instant::now(),
            frame_count: 0,
            fps_update_timer: 0.0,
            current_fps: 0.0,
            window,
        }
    }

    pub fn window(&self) -> &Window {
        self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.renderer.resize(new_size);
            self.camera.aspect = new_size.width as f32 / new_size.height as f32;
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        // Handle mouse scroll separately to access camera
        if let WindowEvent::MouseWheel { delta, .. } = event {
            self.input_handler.handle_mouse_scroll(&mut self.camera, delta);
            return true;
        }

        // Handle cursor movement separately to update camera
        if let WindowEvent::CursorMoved { position, .. } = event {
            self.input_handler.update_mouse_look(&mut self.camera, (position.x, position.y));
            return true;
        }

        self.input_handler.handle_event(event, self.window, self.current_fps, &self.uniforms, &self.camera)
    }

    pub fn update(&mut self) {
        let current_time = Instant::now();
        let dt = current_time.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = current_time;

        // FPS calculation
        self.frame_count += 1;
        self.fps_update_timer += dt;
        
        if self.fps_update_timer >= 1.0 {
            self.current_fps = self.frame_count as f32 / self.fps_update_timer;
            self.frame_count = 0;
            self.fps_update_timer = 0.0;
        }

        // Update camera based on input
        self.input_handler.update_camera(&mut self.camera, dt);
        self.camera.update_target();

        // Update uniforms based on input
        self.input_handler.update_uniforms(&mut self.uniforms, dt);

        // Update uniform matrices and time
        let (view_matrix, projection_matrix) = self.camera.build_view_projection_matrix();
        self.uniforms.view_matrix = view_matrix.into();
        self.uniforms.projection_matrix = projection_matrix.into();
        self.uniforms.camera_pos = self.camera.position.into();
        self.uniforms.time += dt;

        // Send uniforms to GPU
        self.renderer.update_uniforms(&self.uniforms);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render()
    }

    pub fn should_exit(&self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CloseRequested => true,
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    ..
                },
                ..
            } => true,
            _ => false,
        }
    }
}
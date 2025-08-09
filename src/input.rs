use std::collections::HashSet;
use cgmath::InnerSpace;
use winit::{
    event::{WindowEvent, KeyEvent, ElementState, MouseScrollDelta},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::camera::Camera;
use crate::uniforms::Uniforms;

pub struct InputHandler {
    pub keys_pressed: HashSet<KeyCode>,
    pub left_mouse_pressed: bool,
    pub right_mouse_pressed: bool,
    pub middle_mouse_pressed: bool,
    pub last_mouse_pos: (f64, f64),
    pub mouse_captured: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            keys_pressed: HashSet::new(),
            left_mouse_pressed: false,
            right_mouse_pressed: false,
            middle_mouse_pressed: false,
            last_mouse_pos: (0.0, 0.0),
            mouse_captured: false,
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent, window: &Window, current_fps: f32, uniforms: &Uniforms, camera: &Camera) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state,
                    physical_key: PhysicalKey::Code(keycode),
                    ..
                },
                ..
            } => {
                match state {
                    ElementState::Pressed => {
                        self.keys_pressed.insert(*keycode);
                        
                        // Toggle mouse capture with Tab
                        if *keycode == KeyCode::Tab {
                            self.mouse_captured = !self.mouse_captured;
                            if let Err(_) = window.set_cursor_grab(
                                if self.mouse_captured {
                                    winit::window::CursorGrabMode::Confined
                                } else {
                                    winit::window::CursorGrabMode::None
                                }
                            ) {
                                // Fallback if cursor grab not supported
                                self.mouse_captured = false;
                            }
                            window.set_cursor_visible(!self.mouse_captured);
                        }
                        
                        // Toggle FPS display with F key
                        if *keycode == KeyCode::KeyF {
                            println!("FPS: {:.1} | Frame Time: {:.2}ms | {} | Iterations: {} | Power: {:.1} | Speed: {:.1}", 
                                current_fps,
                                1000.0 / current_fps.max(0.1),
                                uniforms.get_fractal_name(),
                                uniforms.fractal_iterations,
                                uniforms.fractal_power,
                                camera.speed
                            );
                        }
                    }
                    ElementState::Released => {
                        self.keys_pressed.remove(keycode);
                    }
                }
                true
            }
            WindowEvent::MouseInput {
                state,
                button,
                ..
            } => {
                let pressed = *state == ElementState::Pressed;
                match button {
                    winit::event::MouseButton::Left => {
                        self.left_mouse_pressed = pressed;
                        
                        // Enable mouse look when left clicking
                        if pressed && !self.mouse_captured {
                            self.mouse_captured = true;
                            if let Err(_) = window.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
                                self.mouse_captured = false;
                            }
                            window.set_cursor_visible(!self.mouse_captured);
                        }
                    }
                    winit::event::MouseButton::Right => {
                        self.right_mouse_pressed = pressed;
                    }
                    winit::event::MouseButton::Middle => {
                        self.middle_mouse_pressed = pressed;
                    }
                    _ => {}
                }
                true
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let scroll_amount = match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y * 0.5,
                    MouseScrollDelta::PixelDelta(pos) => pos.y as f32 * 0.01,
                };
                
                // Return scroll amount for camera speed adjustment
                true
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.mouse_captured {
                    let dx = position.x - self.last_mouse_pos.0;
                    let dy = position.y - self.last_mouse_pos.1;
                    
                    // Store mouse delta for camera update
                    // This will be handled in update_camera
                }
                self.last_mouse_pos = (position.x, position.y);
                true
            }
            _ => false,
        }
    }

    pub fn get_mouse_delta(&self, position: (f64, f64)) -> (f64, f64) {
        if self.mouse_captured {
            (
                position.0 - self.last_mouse_pos.0,
                position.1 - self.last_mouse_pos.1,
            )
        } else {
            (0.0, 0.0)
        }
    }

    pub fn update_camera(&self, camera: &mut Camera, dt: f32) {
        // Variable movement speed based on modifiers
        let base_speed = camera.speed * dt;
        let movement_speed = if self.keys_pressed.contains(&KeyCode::ControlLeft) {
            base_speed * 0.1  // Slow mode with Ctrl
        } else if self.keys_pressed.contains(&KeyCode::ShiftLeft) && !self.keys_pressed.contains(&KeyCode::Space) {
            base_speed * 3.0  // Fast mode with Shift (but not when going up)
        } else {
            base_speed
        };
        
        let forward = (camera.target - camera.position).normalize();
        let right = forward.cross(camera.up).normalize();

        // WASD movement
        if self.keys_pressed.contains(&KeyCode::KeyW) {
            camera.position += forward * movement_speed;
        }
        if self.keys_pressed.contains(&KeyCode::KeyS) {
            camera.position -= forward * movement_speed;
        }
        if self.keys_pressed.contains(&KeyCode::KeyA) {
            camera.position -= right * movement_speed;
        }
        if self.keys_pressed.contains(&KeyCode::KeyD) {
            camera.position += right * movement_speed;
        }
        
        // Vertical movement
        if self.keys_pressed.contains(&KeyCode::Space) {
            let vertical_speed = if self.keys_pressed.contains(&KeyCode::ShiftLeft) {
                base_speed * 0.5  // Slower vertical when shift+space
            } else {
                movement_speed
            };
            camera.position += camera.up * vertical_speed;
        }
        if self.keys_pressed.contains(&KeyCode::KeyC) {
            camera.position -= camera.up * movement_speed;
        }
        
        // Arrow keys for additional movement
        if self.keys_pressed.contains(&KeyCode::ArrowUp) {
            camera.position += forward * movement_speed;
        }
        if self.keys_pressed.contains(&KeyCode::ArrowDown) {
            camera.position -= forward * movement_speed;
        }
        if self.keys_pressed.contains(&KeyCode::ArrowLeft) {
            camera.position -= right * movement_speed;
        }
        if self.keys_pressed.contains(&KeyCode::ArrowRight) {
            camera.position += right * movement_speed;
        }
        
        // Reset camera position
        if self.keys_pressed.contains(&KeyCode::KeyR) {
            camera.reset();
        }
    }

    pub fn update_uniforms(&self, uniforms: &mut Uniforms, dt: f32) {
        if self.keys_pressed.contains(&KeyCode::KeyQ) {
            uniforms.fractal_power = (uniforms.fractal_power - dt * 2.0).max(1.0);
        }
        if self.keys_pressed.contains(&KeyCode::KeyE) {
            uniforms.fractal_power = (uniforms.fractal_power + dt * 2.0).min(20.0);
        }
        
        if self.keys_pressed.contains(&KeyCode::KeyZ) {
            uniforms.fractal_iterations = (uniforms.fractal_iterations as i32 - (dt * 20.0) as i32).max(8) as u32;
        }
        if self.keys_pressed.contains(&KeyCode::KeyX) {
            uniforms.fractal_iterations = (uniforms.fractal_iterations + (dt * 20.0) as u32).min(256);
        }
        
        // Number keys for fractal selection
        if self.keys_pressed.contains(&KeyCode::Digit1) {
            uniforms.fractal_type = 0; // Mandelbulb
        }
        if self.keys_pressed.contains(&KeyCode::Digit2) {
            uniforms.fractal_type = 1; // Julia
        }
        if self.keys_pressed.contains(&KeyCode::Digit3) {
            uniforms.fractal_type = 2; // Menger Sponge
        }
        if self.keys_pressed.contains(&KeyCode::Digit4) {
            uniforms.fractal_type = 3; // Kleinian
        }
        if self.keys_pressed.contains(&KeyCode::Digit5) {
            uniforms.fractal_type = 4; // Apollonian
        }
        if self.keys_pressed.contains(&KeyCode::Digit6) {
            uniforms.fractal_type = 5; // Mandelbox
        }
        if self.keys_pressed.contains(&KeyCode::Digit0) {
            uniforms.fractal_type = 99; // Auto-cycle
        }
    }

    pub fn update_mouse_look(&mut self, camera: &mut Camera, position: (f64, f64)) {
        if self.mouse_captured {
            let (dx, dy) = self.get_mouse_delta(position);
            
            // Right mouse for slower, precise movement
            let sensitivity = if self.right_mouse_pressed {
                camera.sensitivity * 0.3
            } else {
                camera.sensitivity
            };
            
            camera.yaw += dx as f32 * sensitivity;
            camera.pitch -= dy as f32 * sensitivity;
            
            camera.pitch = camera.pitch.clamp(-89.0, 89.0);
            camera.update_target();
        }
        self.last_mouse_pos = position;
    }

    pub fn handle_mouse_scroll(&self, camera: &mut Camera, delta: &MouseScrollDelta) {
        let scroll_amount = match delta {
            MouseScrollDelta::LineDelta(_, y) => *y * 0.5,
            MouseScrollDelta::PixelDelta(pos) => pos.y as f32 * 0.01,
        };
        
        // Scroll wheel adjusts movement speed
        camera.speed = (camera.speed + scroll_amount * 0.5).clamp(0.1, 20.0);
    }
}
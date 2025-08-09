use bytemuck::{Pod, Zeroable};
use cgmath::Matrix4;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub view_matrix: [[f32; 4]; 4],
    pub projection_matrix: [[f32; 4]; 4],
    pub time: f32,
    pub fractal_power: f32,
    pub fractal_iterations: u32,
    pub fractal_type: u32,
    pub camera_pos: [f32; 3],
    pub _padding2: f32,
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            view_matrix: Matrix4::from_scale(1.0).into(),
            projection_matrix: Matrix4::from_scale(1.0).into(),
            time: 0.0,
            fractal_power: 8.0,
            fractal_iterations: 64,
            fractal_type: 0, // Mandelbulb by default
            camera_pos: [0.0, 0.0, -3.0],
            _padding2: 0.0,
        }
    }

    pub fn get_fractal_name(&self) -> &'static str {
        match self.fractal_type {
            0 => "Mandelbulb",
            1 => "Julia Set",
            2 => "Menger Sponge", 
            3 => "Kleinian",
            4 => "Apollonian",
            5 => "Mandelbox",
            99 => "Auto-cycle",
            _ => "Unknown",
        }
    }
}
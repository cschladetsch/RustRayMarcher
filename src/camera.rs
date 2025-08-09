use cgmath::{Matrix4, Vector3, Point3, perspective, Deg, InnerSpace};

pub struct Camera {
    pub position: Vector3<f32>,
    pub target: Vector3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub speed: f32,
    pub sensitivity: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, -3.0),
            target: Vector3::new(0.0, 0.0, 0.0),
            up: Vector3::unit_y(),
            aspect,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
            speed: 2.0,
            sensitivity: 0.002,
            yaw: 90.0,
            pitch: 0.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> (Matrix4<f32>, Matrix4<f32>) {
        let view = Matrix4::look_at_rh(
            Point3::new(self.position.x, self.position.y, self.position.z), 
            Point3::new(self.target.x, self.target.y, self.target.z), 
            self.up
        );
        let proj = perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar);
        (view, proj)
    }

    pub fn update_target(&mut self) {
        let front = Vector3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        ).normalize();
        
        self.target = self.position + front;
    }

    pub fn reset(&mut self) {
        self.position = Vector3::new(0.0, 0.0, -3.0);
        self.yaw = 90.0;
        self.pitch = 0.0;
        self.speed = 2.0;
    }
}
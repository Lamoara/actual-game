use cgmath::{InnerSpace, SquareMatrix};
use winit::keyboard::KeyCode;

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct CameraController {
    speed: f32,
    horizontal: f32,
    vertical: f32,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            horizontal: 0.0,
            vertical: 0.0,
        }
    }

    pub fn handle_key(&mut self, code: KeyCode, is_pressed: bool) -> bool {
        match code {
            KeyCode::KeyW | KeyCode::ArrowUp => {
                self.vertical = if is_pressed {
                    1.0
                } else if self.vertical != -1.0 {
                    0.0
                } else {
                    self.vertical
                };
                true
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => {
                self.horizontal = if is_pressed {
                    -1.0
                } else if self.horizontal != 1.0 {
                    0.0
                } else {
                    self.horizontal
                };
                true
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {
                self.vertical = if is_pressed {
                    -1.0
                } else if self.vertical != 1.0 {
                    0.0
                } else {
                    self.vertical
                };
                true
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {
                self.horizontal = if is_pressed {
                    1.0
                } else if self.horizontal != -1.0 {
                    0.0
                } else {
                    self.horizontal
                };
                true
            }
            _ => false,
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();

        let forward_vec = forward_norm * self.speed * self.vertical;
        camera.eye += forward_vec;
        camera.target += forward_vec;

        let right = forward_norm.cross(camera.up);

        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        camera.eye = camera.target
            - (forward - right * self.horizontal * self.speed).normalize() * forward_mag;
    }
}

use cgmath::Vector3;
use gl::types::GLfloat;

pub struct Cube {
  pub angle: f32,
  pub rotation: Vector3<f32>,
  pub color: [GLfloat; 3],
  pub initial_rotation: f32,
}

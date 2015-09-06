use cgmath::Vector3;

pub struct Cube {
  pub angle: f32,
  pub rotation: Vector3<f32>,
  pub color: [f32; 3],
  pub initial_rotation: f32,
}

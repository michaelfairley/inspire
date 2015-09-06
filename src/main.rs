extern crate sdl2;
extern crate cgmath;
extern crate rand;
#[macro_use]
extern crate glium;
extern crate glium_sdl2;

use std::f32::consts::PI;
use cgmath::Rotation3;
use cgmath::EuclideanVector;

pub mod cube;

const SECONDS_PER_REVOLUTION: f32 = 10.0;
const RADIUS: f32 = 7.0;
const DEPTH: f32 = 13.0;
const HEIGHT: f32 = 1.5;

const NUM_CUBES: i32 = 40;


const COLORS: [[f32; 3]; 5] = [
  [0.25, 0.8, 1.0], // Blue
  [1.0, 0.2, 0.5], // Magenta
  [1.0, 1.0, 0.5], // Yellow
  [0.4, 0.4, 0.4], // Black
  [1.0, 1.0, 1.0], // White
  ];

fn main() {
  use glium_sdl2::DisplayBuild;
  use glium::Surface;

  let sdl_context = sdl2::init().unwrap();

  let video = sdl_context.video().unwrap();

  let gl_attr = video.gl_attr();
  gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
  gl_attr.set_context_version(3, 2);
  gl_attr.set_context_flags().debug().set();

  let window = video
    .window("Inspire", 768/2, 1024/2)
    .resizable()
    // .position_centered()
    // .opengl()
    .build_glium()
    .unwrap();

  let proj = cgmath::perspective(cgmath::deg(90 as f32), 768.0/1024.0, 1.0, 45.0);

  let ambient_strength: f32 = 0.5;

  let program = glium::Program::from_source(&window, include_str!("shaders/fixed.vert"), include_str!("shaders/fixed.frag"), None).unwrap();

  let cubes: Vec<cube::Cube> = (0..NUM_CUBES).map(
    |i|
    cube::Cube {
      angle: i as f32 / NUM_CUBES as f32 * 2.0 * PI,
      rotation: rand::random::<cgmath::Vector3<f32>>().normalize(),
      color: COLORS[i as usize % COLORS.len()],
      initial_rotation: rand::random(),
    }
    ).collect();

  #[derive(Copy, Clone)]
  struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
  }

  implement_vertex!(Vertex, position, normal);

  let verts: &[Vertex] = &[
    // Front
    Vertex { position: [ 1.0,  1.0,  1.0], normal: [ 0.0,  0.0,  1.0] },
    Vertex { position: [ 1.0, -1.0,  1.0], normal: [ 0.0,  0.0,  1.0] },
    Vertex { position: [-1.0,  1.0,  1.0], normal: [ 0.0,  0.0,  1.0] },
    Vertex { position: [-1.0,  1.0,  1.0], normal: [ 0.0,  0.0,  1.0] },
    Vertex { position: [ 1.0, -1.0,  1.0], normal: [ 0.0,  0.0,  1.0] },
    Vertex { position: [-1.0, -1.0,  1.0], normal: [ 0.0,  0.0,  1.0] },

    // Back
    Vertex { position: [ 1.0,  1.0, -1.0], normal: [ 0.0,  0.0, -1.0] },
    Vertex { position: [-1.0,  1.0, -1.0], normal: [ 0.0,  0.0, -1.0] },
    Vertex { position: [ 1.0, -1.0, -1.0], normal: [ 0.0,  0.0, -1.0] },
    Vertex { position: [ 1.0, -1.0, -1.0], normal: [ 0.0,  0.0, -1.0] },
    Vertex { position: [-1.0,  1.0, -1.0], normal: [ 0.0,  0.0, -1.0] },
    Vertex { position: [-1.0, -1.0, -1.0], normal: [ 0.0,  0.0, -1.0] },

    // Left
    Vertex { position: [-1.0,  1.0,  1.0], normal: [-1.0,  0.0,  0.0] },
    Vertex { position: [-1.0, -1.0,  1.0], normal: [-1.0,  0.0,  0.0] },
    Vertex { position: [-1.0,  1.0, -1.0], normal: [-1.0,  0.0,  0.0] },
    Vertex { position: [-1.0,  1.0, -1.0], normal: [-1.0,  0.0,  0.0] },
    Vertex { position: [-1.0, -1.0,  1.0], normal: [-1.0,  0.0,  0.0] },
    Vertex { position: [-1.0, -1.0, -1.0], normal: [-1.0,  0.0,  0.0] },

    // Right
    Vertex { position: [ 1.0,  1.0,  1.0], normal: [ 1.0,  0.0,  0.0] },
    Vertex { position: [ 1.0,  1.0, -1.0], normal: [ 1.0,  0.0,  0.0] },
    Vertex { position: [ 1.0, -1.0,  1.0], normal: [ 1.0,  0.0,  0.0] },
    Vertex { position: [ 1.0, -1.0,  1.0], normal: [ 1.0,  0.0,  0.0] },
    Vertex { position: [ 1.0,  1.0, -1.0], normal: [ 1.0,  0.0,  0.0] },
    Vertex { position: [ 1.0, -1.0, -1.0], normal: [ 1.0,  0.0,  0.0] },

    // Bottom
    Vertex { position: [ 1.0, -1.0,  1.0], normal: [ 0.0, -1.0,  0.0] },
    Vertex { position: [ 1.0, -1.0, -1.0], normal: [ 0.0, -1.0,  0.0] },
    Vertex { position: [-1.0, -1.0,  1.0], normal: [ 0.0, -1.0,  0.0] },
    Vertex { position: [-1.0, -1.0,  1.0], normal: [ 0.0, -1.0,  0.0] },
    Vertex { position: [ 1.0, -1.0, -1.0], normal: [ 0.0, -1.0,  0.0] },
    Vertex { position: [-1.0, -1.0, -1.0], normal: [ 0.0, -1.0,  0.0] },

    // Top
    Vertex { position: [ 1.0,  1.0,  1.0], normal: [ 0.0,  1.0,  0.0] },
    Vertex { position: [-1.0,  1.0,  1.0], normal: [ 0.0,  1.0,  0.0] },
    Vertex { position: [ 1.0,  1.0, -1.0], normal: [ 0.0,  1.0,  0.0] },
    Vertex { position: [ 1.0,  1.0, -1.0], normal: [ 0.0,  1.0,  0.0] },
    Vertex { position: [-1.0,  1.0,  1.0], normal: [ 0.0,  1.0,  0.0] },
    Vertex { position: [-1.0,  1.0, -1.0], normal: [ 0.0,  1.0,  0.0] },
  ];

  let vertex_buffer = glium::VertexBuffer::new(&window, &verts).unwrap();
  let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

  let params = glium::DrawParameters {
    depth_test: glium::DepthTest::IfLessOrEqual,
    depth_range: (0.0, 1.0),
    depth_write: true,
    backface_culling: glium::BackfaceCullingMode::CullCounterClockWise,
    .. Default::default()
  };

  let reflected_params = glium::DrawParameters {
    depth_test: glium::DepthTest::IfLessOrEqual,
    depth_range: (0.0, 1.0),
    depth_write: true,
    backface_culling: glium::BackfaceCullingMode::CullClockWise,
    .. Default::default()
  };

  let mut drawing = true;
  let mut running = true;

  while running {
    for event in sdl_context.event_pump().unwrap().poll_iter() {
      use sdl2::event::Event;
      use sdl2::keyboard::Keycode;

      match event {
        Event::Quit {..}
        | Event::KeyDown { keycode: Some(Keycode::Escape), .. }
        => running = false,
        Event::KeyDown { keycode: Some(Keycode::R), ..}
        => {
        },
        Event::KeyDown { keycode: Some(Keycode::P), ..}
        => drawing = !drawing,
        _ => {}
      }
    }

    let time = sdl_context.timer().unwrap().ticks() as f32 / 1000.0;

    let mut target = window.draw();
    target.clear_color_and_depth((0.96, 0.96, 0.96, 1.0), 1.0);

    for cube in &cubes {

      let current_rotation: cgmath::Quaternion<f32> = cgmath::Quaternion::from_axis_angle(&cube.rotation, cgmath::rad(time + cube.initial_rotation)).normalize();

      let (x, y) = ((time / SECONDS_PER_REVOLUTION) * PI * 2.0 + cube.angle).sin_cos();

      // Regular

      let light_pos: cgmath::Vector3<f32> = cgmath::vec3(0.0, HEIGHT, -DEPTH);

      let trans = cgmath::vec3(x * RADIUS, HEIGHT + y * RADIUS, -DEPTH);
      let model = cgmath::Matrix4::from_translation(&trans);

      let regular_uniforms = uniform! {
        alpha: 1.0f32,
        color: cube.color,
        model: model,
        light_pos: light_pos,
        proj: proj,
        ambient_strength: ambient_strength,
        rotation: cgmath::Matrix4::from(current_rotation),
      };

      target.draw(&vertex_buffer,
                  &indices,
                  &program,
                  &regular_uniforms,
                  &params).unwrap();

      // Reflection
      let light_pos: cgmath::Vector3<f32> = cgmath::vec3(0.0, HEIGHT - (2.7 * RADIUS), -DEPTH);

      let trans = cgmath::vec3(x * RADIUS, HEIGHT + (-2.7 - y) * RADIUS, -DEPTH);
      let flip = cgmath::Matrix4::from(cgmath::Matrix3::from_diagonal(&cgmath::vec3(1.0, - 1.0, 1.0)));
      let model = cgmath::Matrix4::from_translation(&trans) * flip;

      let reflected_uniforms = uniform! {
        alpha: 0.3f32,
        color: cube.color,
        model: model,
        light_pos: light_pos,
        proj: proj,
        ambient_strength: ambient_strength,
        rotation: cgmath::Matrix4::from(current_rotation),
      };

      target.draw(&vertex_buffer,
                  &indices,
                  &program,
                  &reflected_uniforms,
                  &reflected_params).unwrap();
    }

    target.finish().unwrap();
  }
}

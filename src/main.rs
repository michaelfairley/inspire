extern crate sdl2;
extern crate gl;
extern crate cgmath;
extern crate rand;

use gl::types::*;
use std::mem;
use std::ffi::CString;
use std::ptr;
use cgmath::FixedArray;
use cgmath::Rotation3;
use cgmath::EuclideanVector;

pub mod graphics;

fn main() {
  let sdl_context = sdl2::init().unwrap();

  let video = sdl_context.video().unwrap();

  let gl_attr = video.gl_attr();
  gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
  gl_attr.set_context_version(3, 2);
  gl_attr.set_context_flags().debug().set();

  let window = video
    .window("Inspire", 1024, 768)
    .position_centered()
    .opengl()
    .build()
    .unwrap();

  let _context = window.gl_create_context().unwrap();

  gl::load_with(|s| video.gl_get_proc_address(s));

  let trans = cgmath::vec3(1.5 as GLfloat, 3.0, -5.0);
  let model = cgmath::Matrix4::from_translation(&trans);

  let rotation  = rand::random::<cgmath::Vector3<GLfloat>>().normalize();

  let proj = cgmath::perspective(cgmath::deg(90 as GLfloat), 1024.0/768.0, 1.0, 45.0);

  let light_pos: cgmath::Vector3<GLfloat> = cgmath::vec3(0.0, 0.0, -3.0);

  let ambient_strength: GLfloat = 0.5;

  let vert = graphics::compile_shader(include_str!("shaders/fixed.vert"), gl::VERTEX_SHADER);
  let frag = graphics::compile_shader(include_str!("shaders/fixed.frag"), gl::FRAGMENT_SHADER);
  let program = graphics::link_program(vert, frag);

  unsafe {
    gl::UseProgram(program);

    let model_uniform = gl::GetUniformLocation(program, CString::new("model").unwrap().as_ptr());
    gl::UniformMatrix4fv(model_uniform, 1, gl::FALSE, mem::transmute(model.as_fixed()));

    let proj_uniform = gl::GetUniformLocation(program, CString::new("proj").unwrap().as_ptr());
    gl::UniformMatrix4fv(proj_uniform, 1, gl::FALSE, mem::transmute(proj.as_fixed()));

    let light_pos_uniform = gl::GetUniformLocation(program, CString::new("light_pos").unwrap().as_ptr());
    gl::Uniform3fv(light_pos_uniform, 1, mem::transmute(light_pos.as_fixed()));

    let ambient_strength_uniform = gl::GetUniformLocation(program, CString::new("ambient_strength").unwrap().as_ptr());
    gl::Uniform1f(ambient_strength_uniform, ambient_strength);

    gl::UseProgram(0);
  }

  let stride = 9;
  let verts: &[GLfloat] = &[
    // Front
     1.0,  1.0,  1.0, 1.0, 0.0, 0.0,  0.0,  0.0,  1.0,
     1.0, -1.0,  1.0, 1.0, 0.0, 0.0,  0.0,  0.0,  1.0,
    -1.0,  1.0,  1.0, 1.0, 0.0, 0.0,  0.0,  0.0,  1.0,
    -1.0,  1.0,  1.0, 1.0, 0.0, 0.0,  0.0,  0.0,  1.0,
     1.0, -1.0,  1.0, 1.0, 0.0, 0.0,  0.0,  0.0,  1.0,
    -1.0, -1.0,  1.0, 1.0, 0.0, 0.0,  0.0,  0.0,  1.0,

    // Back
     1.0,  1.0, -1.0, 1.0, 0.0, 0.0,  0.0,  0.0, -1.0,
    -1.0,  1.0, -1.0, 1.0, 0.0, 0.0,  0.0,  0.0, -1.0,
     1.0, -1.0, -1.0, 1.0, 0.0, 0.0,  0.0,  0.0, -1.0,
     1.0, -1.0, -1.0, 1.0, 0.0, 0.0,  0.0,  0.0, -1.0,
    -1.0,  1.0, -1.0, 1.0, 0.0, 0.0,  0.0,  0.0, -1.0,
    -1.0, -1.0, -1.0, 1.0, 0.0, 0.0,  0.0,  0.0, -1.0,

    // Left
    -1.0,  1.0,  1.0, 1.0, 0.0, 0.0, -1.0,  0.0,  0.0,
    -1.0, -1.0,  1.0, 1.0, 0.0, 0.0, -1.0,  0.0,  0.0,
    -1.0,  1.0, -1.0, 1.0, 0.0, 0.0, -1.0,  0.0,  0.0,
    -1.0,  1.0, -1.0, 1.0, 0.0, 0.0, -1.0,  0.0,  0.0,
    -1.0, -1.0,  1.0, 1.0, 0.0, 0.0, -1.0,  0.0,  0.0,
    -1.0, -1.0, -1.0, 1.0, 0.0, 0.0, -1.0,  0.0,  0.0,

    // Right
     1.0,  1.0,  1.0, 1.0, 0.0, 0.0,  1.0,  0.0,  0.0,
     1.0,  1.0, -1.0, 1.0, 0.0, 0.0,  1.0,  0.0,  0.0,
     1.0, -1.0,  1.0, 1.0, 0.0, 0.0,  1.0,  0.0,  0.0,
     1.0, -1.0,  1.0, 1.0, 0.0, 0.0,  1.0,  0.0,  0.0,
     1.0,  1.0, -1.0, 1.0, 0.0, 0.0,  1.0,  0.0,  0.0,
     1.0, -1.0, -1.0, 1.0, 0.0, 0.0,  1.0,  0.0,  0.0,

    // Bottom
     1.0, -1.0,  1.0, 1.0, 0.0, 0.0,  0.0, -1.0,  0.0,
     1.0, -1.0, -1.0, 1.0, 0.0, 0.0,  0.0, -1.0,  0.0,
    -1.0, -1.0,  1.0, 1.0, 0.0, 0.0,  0.0, -1.0,  0.0,
    -1.0, -1.0,  1.0, 1.0, 0.0, 0.0,  0.0, -1.0,  0.0,
     1.0, -1.0, -1.0, 1.0, 0.0, 0.0,  0.0, -1.0,  0.0,
    -1.0, -1.0, -1.0, 1.0, 0.0, 0.0,  0.0, -1.0,  0.0,

    // Top
     1.0,  1.0,  1.0, 1.0, 0.0, 0.0,  0.0,  1.0,  0.0,
    -1.0,  1.0,  1.0, 1.0, 0.0, 0.0,  0.0,  1.0,  0.0,
     1.0,  1.0, -1.0, 1.0, 0.0, 0.0,  0.0,  1.0,  0.0,
     1.0,  1.0, -1.0, 1.0, 0.0, 0.0,  0.0,  1.0,  0.0,
    -1.0,  1.0,  1.0, 1.0, 0.0, 0.0,  0.0,  1.0,  0.0,
    -1.0,  1.0, -1.0, 1.0, 0.0, 0.0,  0.0,  1.0,  0.0,
  ];

  let mut vertex_buffer = 0;

  unsafe {
    gl::GenBuffers(1, &mut vertex_buffer);

    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (verts.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   mem::transmute(&verts[0]),
                   gl::STATIC_DRAW);
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  }

  let mut vao = 0;

  unsafe {
    gl::GenVertexArrays(1, &mut vao);

    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);

    let pos_attr = gl::GetAttribLocation(program,
                                         CString::new("position").unwrap().as_ptr()) as GLuint;
    gl::EnableVertexAttribArray(pos_attr);
    gl::VertexAttribPointer(pos_attr, 3, gl::FLOAT, gl::FALSE, stride * mem::size_of::<GLfloat>() as GLsizei, ptr::null());

    let color_attr = gl::GetAttribLocation(program,
                                           CString::new("color").unwrap().as_ptr()) as GLuint;
    gl::EnableVertexAttribArray(color_attr);
    gl::VertexAttribPointer(color_attr, 3, gl::FLOAT, gl::FALSE, stride * mem::size_of::<GLfloat>() as GLsizei, mem::transmute(3 * mem::size_of::<GLfloat>()));

    let normal_attr = gl::GetAttribLocation(program,
                                           CString::new("normal").unwrap().as_ptr()) as GLuint;
    gl::EnableVertexAttribArray(normal_attr);
    gl::VertexAttribPointer(normal_attr, 3, gl::FLOAT, gl::FALSE, stride * mem::size_of::<GLfloat>() as GLsizei, mem::transmute(6 * mem::size_of::<GLfloat>()));

    gl::BindVertexArray(0);
  }

  unsafe {
    gl::Enable(gl::DEPTH_TEST);
    gl::DepthMask(gl::TRUE);
    gl::DepthFunc(gl::LEQUAL);
    gl::DepthRange(0.0, 1.0);

    gl::Enable(gl::CULL_FACE);
    gl::CullFace(gl::BACK);
    gl::FrontFace(gl::CW);
  }

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
        }
        _ => {}
      }
    }

    let time = sdl_context.timer().unwrap().get_ticks() as f32 / 1000.0;

    unsafe {
      gl::ClearColor(0.0, 0.0, 0.0, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

      gl::UseProgram(program);

      let rotation_uniform = gl::GetUniformLocation(program, CString::new("rotation").unwrap().as_ptr());
      let current_rotation: cgmath::Quaternion<GLfloat> = cgmath::Quaternion::from_axis_angle(&rotation, cgmath::rad(time)).normalize();
      gl::UniformMatrix4fv(rotation_uniform, 1, gl::FALSE, mem::transmute(cgmath::Matrix4::from(current_rotation).as_fixed()));

      gl::BindVertexArray(vao);
      gl::DrawArrays(gl::TRIANGLES, 0, verts.len() as i32 / stride);
      gl::BindVertexArray(0);

      gl::UseProgram(0);

      window.gl_swap_window();
    }
  }
}

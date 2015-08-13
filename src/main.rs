extern crate sdl2;
extern crate gl;

use gl::types::*;
use std::mem;
use std::ffi::CString;
use std::ptr;

pub mod graphics;

fn main() {
  let sdl_context = sdl2::init().unwrap();

  let video = sdl_context.video().unwrap();

  let gl_attr = video.gl_attr();
  gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
  gl_attr.set_context_version(3, 2);
  gl_attr.set_context_flags().debug().set();

  let window = video
    .window("Lampasas", 1024, 768)
    .position_centered()
    .opengl()
    .build()
    .unwrap();

  let _context = window.gl_create_context().unwrap();

  gl::load_with(|s| video.gl_get_proc_address(s));

  let vert = graphics::compile_shader(include_str!("shaders/fixed.vert"), gl::VERTEX_SHADER);
  let frag = graphics::compile_shader(include_str!("shaders/fixed.frag"), gl::FRAGMENT_SHADER);
  let program = graphics::link_program(vert, frag);

  let verts: [GLfloat; 9] = [
    0.0, 0.5, 0.0,
    0.5, -0.5, 0.0,
    -0.5, -0.5, 0.0,
    ];

  let mut vbo = 0;

  unsafe {
    gl::GenBuffers(1, &mut vbo);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (verts.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   mem::transmute(&verts[0]),
                   gl::STATIC_DRAW);
  }

  let mut vao = 0;

  unsafe {
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);
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

    unsafe {
      gl::ClearColor(0.0, 0.0, 0.0, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);

      gl::UseProgram(program);

      gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
      let pos_attr = gl::GetAttribLocation(program,
                                           CString::new("position").unwrap().as_ptr()) as GLuint;
      gl::EnableVertexAttribArray(pos_attr);
      gl::VertexAttribPointer(pos_attr, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());

      gl::DrawArrays(gl::TRIANGLES, 0, 3);

      gl::DisableVertexAttribArray(pos_attr);
      gl::UseProgram(0);

      window.gl_swap_window();
    }
  }
}

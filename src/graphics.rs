extern crate gl;

use std::ffi::CString;
use std::ptr;
use std::str;
use gl::types::*;

pub fn compile_shader(src: &str, typ: GLenum) -> GLuint {
  let shader;
  unsafe {
    shader = gl::CreateShader(typ);
    let c_str = CString::new(src.as_bytes()).unwrap();
    gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
    gl::CompileShader(shader);

    let mut status = gl::FALSE as GLint;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
    if status == gl::FALSE as GLint {
      let mut len = 0;
      gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
      let mut buf = Vec::with_capacity(len as usize);
      buf.set_len((len as usize) - 1);
      gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
      panic!("{}", str::from_utf8(&buf).unwrap());
    }
  }
  shader
}

pub fn link_program(vert: GLuint, frag: GLuint) -> GLuint {
  unsafe {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vert);
    gl::AttachShader(program, frag);
    gl::LinkProgram(program);

    let mut status = gl::FALSE as GLint;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

    if status != gl::TRUE as GLint {
      let mut len: GLint = 0;
      gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
      let mut buf = Vec::with_capacity(len as usize);
      buf.set_len((len as usize) - 1);
      gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
      panic!("{}", str::from_utf8(&buf).unwrap());
    }
    program
  }
}

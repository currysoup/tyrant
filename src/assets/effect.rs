use std::ffi::CString;
use std::fs::File;
use std::ops::Drop;
use std::io::prelude::*;
use std::path::Path;
use std::ptr;
use std::str;

use gl;
use gl::types::*;

use super::{Asset, AssetError, Result};

pub struct Effect {
    pub program: GLuint,
}

impl Effect {
    pub fn new(program: GLuint) -> Effect {
        Effect {
            program: program,
        }
    }

    pub fn apply(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }
}

impl Drop for Effect {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}

impl Asset for Effect {
    fn load<R: Read + Seek>(reader: R) -> Result<Effect> {
        let mut buffer = String::new();
        let vertex_id;
        let fragment_id;

        vertex_id = compile_shader(&buffer, gl::VERTEX_SHADER);

        buffer.clear();

        fragment_id = compile_shader(&buffer, gl::FRAGMENT_SHADER);

        let program = link_program(vertex_id, fragment_id);
        unsafe {
            gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());

            gl::DeleteShader(vertex_id);
            gl::DeleteShader(fragment_id);
        }

        Effect {
            program: program,
        }
    }
}

fn compile_shader(code: &str, shader_type: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(shader_type);

        let c_str = CString::new(code.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(buf.as_slice()).ok().expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(buf.as_slice()).ok().expect("ProgramInfoLog not valid utf8"));
        }
        program
    }
}

use std::mem;
use std::ptr;

use gl;
use gl::types::*;

use math::Vector3;

use super::{ VaoHandle, VboHandle, Vertex };

pub struct VertexPosition {
    pub pos: Vector3,
}

impl VertexPosition {
    pub fn new(pos: Vector3) -> VertexPosition {
        VertexPosition {
            pos: pos,
        }
    }

    pub fn from_parts(x: f32, y: f32, z: f32) -> VertexPosition {
        VertexPosition {
            pos: Vector3::new(x, y, z),
        }
    }
}

impl Vertex for VertexPosition {
    fn gen_vao(vbo: &VboHandle) -> VaoHandle {
        let VboHandle(vbo) = *vbo;

        let mut vao: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            let stride = mem::size_of::<VertexPosition>();

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,
                                    3,
                                    gl::FLOAT,
                                    gl::FALSE as GLboolean,
                                    stride as GLint,
                                    ptr::null());
        }

        VaoHandle(vao)
    }
}

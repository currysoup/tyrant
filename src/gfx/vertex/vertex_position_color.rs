use std::mem;
use std::ptr;

use gl;
use gl::types::*;

use math::{Vector3, Vector4};
use super::{ VaoHandle, VboHandle, Vertex };

pub struct VertexPositionColor {
    pub pos: Vector3,
    pub col: Vector4,
}

impl VertexPositionColor {
    pub fn new (pos: Vector3, col: Vector4) -> VertexPositionColor {
        VertexPositionColor {
            pos: pos,
            col: col,
        }
    }
}

impl Vertex for VertexPositionColor {
    fn gen_vao(vbo: &VboHandle) -> VaoHandle {
        let VboHandle(vbo) = *vbo;

        let mut vao: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            let stride = mem::size_of::<VertexPositionColor>();

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,
                                    3,
                                    gl::FLOAT,
                                    gl::FALSE as GLboolean,
                                    stride as GLint,
                                    ptr::null());

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1,
                                    4,
                                    gl::FLOAT,
                                    gl::FALSE as GLboolean,
                                    stride as GLint,
                                    mem::transmute(3_usize * 4_usize));
        }

        VaoHandle(vao)
    }
}

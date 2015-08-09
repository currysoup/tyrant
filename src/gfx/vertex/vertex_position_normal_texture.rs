use std::mem;
use std::ptr;

use gl;
use gl::types::*;

use math::Vector2;
use math::Vector3;

use super::{ VaoHandle, VboHandle, Vertex };

pub struct VertexPositionNormalTexture {
    pub pos: Vector3,
    pub norm: Vector3,
    pub tex: Vector2,
}

impl VertexPositionNormalTexture {
    pub fn new(pos: Vector3, norm: Vector3, tex: Vector2) -> VertexPositionNormalTexture {
        VertexPositionNormalTexture {
            pos: pos,
            norm: norm,
            tex: tex,
        }
    }
}

impl Vertex for VertexPositionNormalTexture {
    fn gen_vao(vbo: &VboHandle) -> VaoHandle {
        let VboHandle(vbo) = *vbo;

        let mut vao: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            let stride = mem::size_of::<VertexPositionNormalTexture>();

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,
                                    3,
                                    gl::FLOAT,
                                    gl::FALSE as GLboolean,
                                    stride as GLint,
                                    ptr::null());

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1,
                                    3,
                                    gl::FLOAT,
                                    gl::FALSE as GLboolean,
                                    stride as GLint,
                                    mem::transmute(3_usize * 4_usize));

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(2,
                                    2,
                                    gl::FLOAT,
                                    gl::FALSE as GLboolean,
                                    stride as GLint,
                                    mem::transmute(6_usize * 4_usize));
        }

        VaoHandle(vao)
    }
}

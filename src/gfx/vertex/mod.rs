mod vertex_position;
mod vertex_position_color;
mod vertex_position_normal;
mod vertex_position_normal_texture;

use std::mem;

use gl;
use gl::types::*;

pub use self::vertex_position::VertexPosition;
pub use self::vertex_position_color::VertexPositionColor;
pub use self::vertex_position_normal::VertexPositionNormal;
pub use self::vertex_position_normal_texture::VertexPositionNormalTexture;

/// Simple handle for the vertex buffer object.
pub struct VboHandle(pub GLuint);

/// Simple handle for the vertex array object.
pub struct VaoHandle(pub GLuint);

// ****************
// * Vertex trait *
// ****************

/// Trait defines a type as a vertex type, provides a funciton which will create a VaoHandle for a given
/// vbo pair. Note: The layout of the shader must be as the vertex format expects!
pub trait Vertex {
    fn gen_vao(vbo: &VboHandle) -> VaoHandle;
}

// **********************
// * Vertex Array trait *
// **********************

/// Trait defines a container of vertices, allows the implementation of the `to_vbo` function which
/// consumes the container and produces a VboHandle.
pub trait VertexArray<T: Vertex> {
    /// Consume the container - turning it into a GL vbo
    fn to_vbo(self) -> VboHandle;
    fn len(&self) -> usize;
}

impl<T: Vertex> VertexArray<T> for Vec<T> {
    fn to_vbo(self) -> VboHandle {
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (self.len() * mem::size_of::<T>()) as GLsizeiptr,
                           mem::transmute(&self[0]),
                           gl::STATIC_DRAW);
        }

        VboHandle(vbo)
    }

    fn len(&self) -> usize {
        self.len()
    }
}

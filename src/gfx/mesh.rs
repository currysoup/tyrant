use std::ops::Drop;

use gl;

use effect::Effect;
use texture::Texture;
use vertex::{VaoHandle, VboHandle, Vertex, VertexArray};

pub struct Mesh {
    effect: Effect,

    texture: Texture,

    vbo: VboHandle,
    vertex_count: usize,
    vao: VaoHandle,
}

impl Mesh {
    pub fn new<T, D>(tex: Texture, effect: Effect, data: T) -> Mesh
        where T: VertexArray<D>, D: Vertex {
            let vertex_count = data.len();
            let vbo = data.to_vbo();
            let vao = D::gen_vao(&vbo);

            Mesh {
                effect: effect,
                texture: tex,
                vbo: vbo,
                vertex_count: vertex_count,
                vao: vao,
            }
        }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        let VboHandle(vbo) = self.vbo;
        let VaoHandle(vao) = self.vao;
        unsafe {
            gl::DeleteBuffers(1, &vbo);
            gl::DeleteVertexArrays(1, &vao);
        }
    }
}

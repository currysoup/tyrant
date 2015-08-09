#![feature(convert)]

extern crate image;
extern crate gl;
extern crate glfw;

extern crate content;
extern crate math;

pub mod effect;
pub mod mesh;
pub mod texture;
pub mod window;
pub mod vertex;
pub mod debug_font;

use vertex::{VaoHandle, VboHandle}

pub struct Renderer {
    visibles: Visibles,
    //targets: Vec<RenderTarget>,
    //passes: Vec<Pass>,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            visibles: Visibles::new(),
        }
    }

    pub fn clear_visibles(&mut self) {
        self.visibles.textures.clear();
        self.visibles.objects.clear();
    }

    pub fn render(&self) {
        //for target in self.targets // Render Targets
        //for pass in self.passes // Render Passes
        for effect in self.visibles {
            gl::BindVertexArray(material);
            gl::UseProgram(material);

            for texture in self.visibles.textures {
                gl::BindTexture(gl::TEXTURE_2D, texture);

                for object in self.visibles.objects {
                    gl::BindBuffer(gl::ARRAY_BUFFER, object.vbo);

                    for elements in object {
                        // Write Uniform Data:
                        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, elements);

                        gl::DrawElementsBaseVertex(
                            gl::TRIANGLES,
                            index_count,
                            gl::UNSIGNED_SHORT,
                            index_offset,
                            base_object);
                    }
                }
            }
        }
    }

    pub fn add_material(&mut self, mat: Material) {

    }

    pub fn add_texture(&mut self) {

    }

    pub fn add_object(&mut self, vbo: Vbo, elements: Ebo) {
        let Vbo(vbo) = vbo;
        let Ebo(ebo) = ebo;

        // Find vertex buffers
    }
}

pub struct Visibles {
    pub materials: Vec<VisibleMaterial>,
    pub texture: Vec<VisibleTexture>,
}

// Should be possible to make the following structures more cache friendly by sharing `VisibleX`
// buffers.
pub struct VisibleMaterial {
    pub id: GLuint,
    pub texture: Vec<VisibleTexture>,
}

pub struct VisibleTexture {
    pub id: GLuint,
    pub meshes: Vec<VisibleMesh>,
}

pub struct VisibleMesh {
    pub vbo: Vbo,
    pub elements: Vec<Ebo>,
}

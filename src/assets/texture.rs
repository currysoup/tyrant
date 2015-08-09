use std::io::{Read, Seek};
use std::mem;

use image;
use gl;
use gl::types::{GLint, GLuint};

use super::{Asset, AssetError, Result};

pub struct Texture {
    handle: GLuint,
}

impl Asset for Texture {
    fn load<R: Read + Seek>(reader: R) -> Result<Self> {
        let img = match image::load(reader, image::ImageFormat::PNG) {
            Ok(img) => img,
            Err(why) => return Err(AssetError::Image(why)),
        };

        let img = match img {
            image::ImageRgb8(rgb) => rgb,
            _ => panic!("Unsupported texture format!"),
        };

        let (w, h) = img.dimensions();
        let data = img.into_raw();

        let mut tex: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::TexImage2D(gl::TEXTURE_2D,
                          0, gl::RGBA as GLint,
                          w as i32,
                          h as i32,
                          0,
                          gl::RGBA,
                          gl::UNSIGNED_BYTE,
                          mem::transmute(&data));
        }

        Ok(Texture {handle: tex})
    }
}

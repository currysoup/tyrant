use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;

use image::ImageError;

pub use self::texture::Texture;

mod texture;

pub type Result<T> = StdResult<T, AssetError>;

// Internal workings of asset loads is private to this module
trait Asset {
    fn load<R: Read + Seek>(reader: R) -> Result<Self>;
}

#[derive(Debug)]
pub enum AssetError {
    Image(ImageError),
}

pub struct AssetManager {
    base_dir: PathBuf,
    assets: HashMap<String, i32>,
}

impl AssetManager {
    pub fn new<P: AsRef<Path>>(dir: P) -> AssetManager {
        let mut base_dir = PathBuf::new();
        base_dir.push(dir);

        AssetManager {
            base_dir: base_dir,
            assets: HashMap::new(),
        }
    }

    pub fn load<T: Asset, P: AsRef<Path>>(&self, path: P) -> T {
        let mut full_path = PathBuf::new();
        full_path.push(path);

        let file = File::open(full_path).unwrap();
        T::load(file).unwrap()
    }
}

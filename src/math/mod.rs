pub use self::matrix::Matrix;

pub use self::vector2::Vector2;
pub use self::vector3::Vector3;
pub use self::vector4::Vector4;

mod matrix;

mod vector2;
mod vector3;
mod vector4;

// Might implement this properly later
pub type Color = Vector4;

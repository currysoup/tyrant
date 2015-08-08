use math::{Matrix, Vector3};

pub struct Camera {
    pos: Vector3,
    target: Vector3,
    look_at: Matrix,
    projection: Matrix,
}

impl Camera {
    pub fn new(pos: Vector3, direction: Vector3) -> Camera {
        let target = pos + direction.normalize();
        Camera {
            pos: pos,
            target: target,
            look_at: Matrix::look_at(pos, target, Vector3::unit_y()),
            projection: Matrix::orthographic(800.0, 600.0, 1.0, 100.0),
        }
    }

    pub fn translate(&mut self, dir: Vector3) {
        unimplemented!();
    }

    pub fn rotate(&mut self, rot: Rotation) {
        unimplemented!();
    }
}

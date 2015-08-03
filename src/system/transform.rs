use super::MAX_ENTITIES;

pub struct TransformSystem {
    count: usize,
    x: Box<[f32; MAX_ENTITIES]>,
    y: Box<[f32; MAX_ENTITIES]>,
    z: Box<[f32; MAX_ENTITIES]>,
    vel_x: Box<[f32; MAX_ENTITIES]>,
    vel_y: Box<[f32; MAX_ENTITIES]>,
    vel_z: Box<[f32; MAX_ENTITIES]>,
}

impl TransformSystem {
    pub fn new() -> TransformSystem {
        TransformSystem {
            count: 0,
            x: Box::new([0.0; MAX_ENTITIES]),
            y: Box::new([0.0; MAX_ENTITIES]),
            z: Box::new([0.0; MAX_ENTITIES]),
            vel_x: Box::new([0.0; MAX_ENTITIES]),
            vel_y: Box::new([0.0; MAX_ENTITIES]),
            vel_z: Box::new([0.0; MAX_ENTITIES]),
        }
    }

    fn update(&mut self, time: &FrameTime) {
        for i in 0..self.count {
            self.x[i] += self.vel_x[i] * time.elapsed_s();
            self.y[i] += self.vel_y[i] * time.elapsed_s();
            self.z[i] += self.vel_z[i] * time.elapsed_s();
        }
    }

    pub fn set_position(&mut self, idx: usize, x: f32, y: f32, z: f32) {
        self.x[idx] = x;
        self.y[idx] = y;
        self.z[idx] = z;
    }
}

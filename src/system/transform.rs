use frametime::FrameTime;
use math::Vector3;
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

    pub fn update(&mut self, time: &FrameTime) {
        for i in 0..self.count {
            self.x[i] += self.vel_x[i] * time.elapsed_s();
            self.y[i] += self.vel_y[i] * time.elapsed_s();
            self.z[i] += self.vel_z[i] * time.elapsed_s();
        }
    }

    pub fn set_position(&mut self, idx: usize, pos: Vector3) {
        self.x[idx] = pos.x;
        self.y[idx] = pos.y;
        self.z[idx] = pos.z;
    }

    pub fn set_velocity(&mut self, idx: usize, vel: Vector3) {
        self.vel_x[idx] = vel.x;
        self.vel_y[idx] = vel.y;
        self.vel_z[idx] = vel.z;
    }

    pub fn get_position(&self, idx: usize) -> Vector3 {
        Vector3::new(self.x[idx], self.y[idx], self.z[idx])
    }

    pub fn get_velocity(&self, idx: usize) -> Vector3 {
        Vector3::new(self.vel_x[idx], self.vel_y[idx], self.vel_z[idx])
    }

    pub fn next_index(&mut self) -> usize{
        let val = self.count;
        self.count += 1;

        val
    }
}

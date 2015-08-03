pub struct Rotation {
    pitch: f32,
    yaw: f32,
    roll: f32,
}

impl Rotation {
    pub fn new(pitch: f32, yaw: f32, roll: f32) -> Rotation {
        Rotation {
            pitch: pitch,
            yaw: yaw,
            roll: roll,
        }
    }
}

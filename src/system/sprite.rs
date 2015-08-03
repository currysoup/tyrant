use super::MAX_ENTITIES;

type SpriteHandle = i32;

pub struct SpriteSystem {
    count: usize,
    sprites: Box<[SpriteHandle; MAX_ENTITIES]>,
}

impl SpriteSystem {
    pub fn new() -> SpriteSystem {
         SpriteSystem {
            count: 0,
            sprites: Box::new([0; MAX_ENTITIES]),
        }
    }

    fn update(&mut self, time: &FrameTime) {
    }
}

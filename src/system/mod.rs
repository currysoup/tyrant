use frametime::FrameTime;

use self::transform::TransformSystem;

mod transform;
mod sprite;

const MAX_ENTITIES: usize = 8192;

pub struct EntitySystem {
    index: Box<[SystemRow; MAX_ENTITIES]>,
    transforms: TransformSystem,
}

impl EntitySystem {
    pub fn new() -> EntitySystem {
        EntitySystem {
            index: Box::new([SystemRow::new(); MAX_ENTITIES]),
            transforms: TransformSystem::new(),
        }
    }

    pub fn update(&mut self) {
        transforms.update();
    }

    pub fn new_entity(&self) -> EntityBuilder {

    }

    pub fn set_position(&mut self, entity: usize, pos: Vector3) {
        let idx = self.index[entity].transform;
        transforms.set_position(idx, pos.x, pos.y, pos.z);
    }
}

/// Entity builders are used to simplify the process of atomically creating new entities.
pub struct EntityBuilder {
    transformable: Option<Vector3>,
    system: &mut EntitySystem,
}

impl EntityBuilder {
    pub fn new(system: &mut EntitySystem) -> EntityBuilder {
        EntityBuilder {
            transformable: None,
            system: system,
        }
    }

    pub fn transformable(&mut self, initial_pos: Vector3) -> &mut EntityBuilder {
        self.transformable = Some(initial_pos);
        self
    }
}

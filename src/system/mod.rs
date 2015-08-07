use frametime::FrameTime;
use math::Vector3;

use self::transform::TransformSystem;

mod transform;
mod sprite;

const MAX_ENTITIES: usize = 8192;

#[derive(Clone, Copy, Debug)]
struct SystemRow {
    transform: usize,
    sprite: usize,
}

impl SystemRow {
    pub fn new() -> SystemRow {
        SystemRow {
            transform: 8193, // MAX_ENTITIES + 1,
            sprite: 8193, // MAX_ENTITES + 1,
        }
    }
}

pub struct EntitySystem {
    index: Box<[SystemRow; MAX_ENTITIES]>,
    free_slots: Vec<usize>,
    next_slot: usize,
    transforms: TransformSystem,
}

impl EntitySystem {
    pub fn new() -> EntitySystem {
        EntitySystem {
            index: Box::new([SystemRow::new(); MAX_ENTITIES]),
            free_slots: Vec::new(),
            next_slot: 0,
            transforms: TransformSystem::new(),
        }
    }

    /// Update all the systems.
    pub fn update(&mut self, time: &FrameTime) {
        self.transforms.update(time);
    }

    /// Begin building a new entity using an `EntityBuilder`
    pub fn new_entity(&mut self) -> EntityBuilder {
        EntityBuilder::new(self)
    }

    /// Used to insert an entity into the system.
    fn add_entity(&mut self, t_idx: Option<usize>) -> usize {
        let slot = match self.free_slots.pop() {
            Some(slot) => slot,
            None => {
                let slot = self.next_slot;
                self.next_slot += 1;
                slot
            },
        };

        self.index[slot] = SystemRow {
            transform: t_idx.unwrap_or(MAX_ENTITIES + 1),
            sprite: MAX_ENTITIES + 1,
        };

        slot
    }

    pub fn set_position(&mut self, entity: usize, pos: Vector3) {
        let idx = self.index[entity].transform;
        self.transforms.set_position(idx, pos);
    }

    pub fn set_velocity(&mut self, entity: usize, vel: Vector3) {
        let idx = self.index[entity].transform;
        self.transforms.set_velocity(idx, vel);
    }

    pub fn get_position(&self, entity: usize) -> Vector3 {
        let idx = self.index[entity].transform;
        self.transforms.get_position(idx)
    }

    pub fn get_velocity(&self, entity: usize) -> Vector3 {
        let idx = self.index[entity].transform;
        self.transforms.get_velocity(idx)
    }
}

/// Entity builders are used to simplify the process of atomically creating new entities.
pub struct EntityBuilder<'a> {
    transformable: Option<Vector3>,
    system: &'a mut EntitySystem,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(system: &'a mut EntitySystem) -> EntityBuilder<'a> {
        EntityBuilder {
            transformable: None,
            system: system,
        }
    }

    pub fn transformable(&mut self, initial_pos: Vector3) -> &mut EntityBuilder<'a> {
        self.transformable = Some(initial_pos);
        self
    }

    pub fn build(&mut self) -> usize {
        // Atomically build entity
        let mut t_idx = None;
        if let Some(pos) = self.transformable {
            let idx = self.system.transforms.next_index();
            self.system.transforms.set_position(idx, pos);
            t_idx = Some(idx);
        }

        self.system.add_entity(t_idx)
    }
}

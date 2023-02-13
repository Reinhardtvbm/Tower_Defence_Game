use bevy::prelude::Vec2;

use crate::resources::entity::TowerEntity;

#[derive(Debug)]
pub struct Tile {
    entity: Option<TowerEntity>,
    spawn_position: Vec2,
}

impl Tile {
    /// creates a new Tile struct with no TowerEntity
    pub fn new(spawn_position: Vec2) -> Self {
        Self {
            entity: None,
            spawn_position,
        }
    }

    pub fn get_spawn_position(&self) -> Vec2 {
        self.spawn_position
    }

    /// changes the Tile's entity to None
    pub fn set_entity_none(&mut self) {
        self.entity = None;
    }

    /// changes the Tile's entity to the given entity
    pub fn set_entity(&mut self, entity: TowerEntity) {
        self.entity = Some(entity);
    }
}

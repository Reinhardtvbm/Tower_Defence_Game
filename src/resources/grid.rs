use std::collections::HashMap;

use bevy::prelude::*;

use super::entity::TowerEntity;

/// Grid that represents the tiles on the map
#[derive(Resource)]
pub struct Grid(HashMap<GridCoord, Tile>);

impl Grid {
    pub fn new(size: GridSize) -> Self {
        let width = size.width;
        let height = size.height;

        let mut grid_map = HashMap::new();

        (0..height).into_iter().for_each(|row| {
            (0..width).into_iter().for_each(|column| {
                grid_map.insert(GridCoord(row, column), Tile { entity: None });
            });
        });

        Self(grid_map)
    }
}

pub struct Tile {
    entity: Option<TowerEntity>,
}

impl Tile {
    /// creates a new Tile struct with no TowerEntity
    pub fn new() -> Self {
        Self { entity: None }
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

#[derive(Hash, PartialEq, Eq)]
pub struct GridCoord(usize, usize);

pub struct GridSize {
    pub width: usize,
    pub height: usize,
}

use std::collections::{hash_map::Keys, HashMap};

use bevy::prelude::*;

use super::entity::TowerEntity;

/// Grid that represents the tiles on the map
#[derive(Resource, Debug)]
pub struct Grid {
    tiles: HashMap<GridCoord, Tile>,
    width: usize,
    height: usize,
}

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

        Self {
            tiles: grid_map,
            width,
            height,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    // HashMap functions:

    pub fn get(&self, grid_coord: &GridCoord) -> Option<&Tile> {
        self.tiles.get(grid_coord)
    }

    pub fn get_mut(&mut self, grid_coord: &GridCoord) -> Option<&mut Tile> {
        self.tiles.get_mut(grid_coord)
    }

    pub fn get_key_value(&self, grid_coord: &GridCoord) -> Option<(&GridCoord, &Tile)> {
        self.tiles.get_key_value(grid_coord)
    }

    pub fn contains_key(&self, grid_coord: &GridCoord) -> bool {
        self.tiles.contains_key(grid_coord)
    }

    pub fn insert(&mut self, grid_coord: GridCoord, tile: Tile) -> Option<Tile> {
        self.tiles.insert(grid_coord, tile)
    }

    pub fn remove(&mut self, grid_coord: GridCoord) -> Option<Tile> {
        self.tiles.remove(&grid_coord)
    }

    pub fn clear(&mut self) {
        self.tiles.clear()
    }

    pub fn keys(&self) -> Keys<'_, GridCoord, Tile> {
        self.tiles.keys()
    }

    // ===========================================================================================
}

#[derive(Debug)]
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

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct GridCoord(usize, usize);

pub struct GridSize {
    pub width: usize,
    pub height: usize,
}

use std::collections::{hash_map::Keys, HashMap};

use bevy::prelude::*;

use super::entity::TowerEntity;

/// Grid that represents the tiles on the map
#[derive(Resource, Debug)]
pub struct Grid {
    tiles: HashMap<GridCoord, Tile>,
    size: GridSize,
    cell_length: f32,
    y_offset: f32,
    x_offset: f32,
}

impl Grid {
    pub fn new(size: GridSize, cell_length: f32, x_offset: f32, y_offset: f32) -> Self {
        let width = size.width;
        let height = size.height;

        let mut grid_map = HashMap::new();

        (0..height).into_iter().for_each(|row| {
            (0..width).into_iter().for_each(|column| {
                // calculate the x position of the new highlight block
                let x_pos = (cell_length / 2.0) + (column as f32 * cell_length) + x_offset;

                // calculate the x position of the new highlight block
                let y_pos = (cell_length / 2.0) + (row as f32 * cell_length) + y_offset
                    - (cell_length / 2.0 * height as f32);

                grid_map.insert(
                    GridCoord(column, row),
                    Tile::new(Vec2 { x: x_pos, y: y_pos }),
                );
            });
        });

        Self {
            tiles: grid_map,
            size,
            cell_length,
            x_offset,
            y_offset,
        }
    }

    pub fn get_width(&self) -> usize {
        self.size.width
    }

    pub fn get_height(&self) -> usize {
        self.size.height
    }

    pub fn get_cell_length(&self) -> f32 {
        self.cell_length
    }

    pub fn get_x_offset(&self) -> f32 {
        self.x_offset
    }

    pub fn get_y_offset(&self) -> f32 {
        self.y_offset
    }

    pub fn get_cell_mouse_pos(&self, mouse_x: f32, mouse_y: f32) -> Option<(&GridCoord, &Tile)> {
        let x = ((mouse_x - self.x_offset) / self.cell_length).floor() as usize;
        let y = ((mouse_y + self.y_offset + ((self.cell_length * self.size.height as f32) / 2.0))
            / self.cell_length)
            .floor() as usize;

        let grid_coord = GridCoord(x, y);

        self.get_key_value(&grid_coord)
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

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct GridCoord(pub usize, pub usize);

#[derive(Debug)]
pub struct GridSize {
    pub width: usize,
    pub height: usize,
}

use std::{
    collections::{hash_map::Keys, HashMap},
    slice::{Iter, IterMut},
};

use bevy::prelude::*;

use crate::resources::entity::{Tower, TowerEntity};

use super::{error::TowerSpawnErr, grid_coord::GridCoord, grid_size::GridSize, tile::Tile};

/// Grid that represents the tiles on the map
#[derive(Resource, Debug)]
pub struct Grid {
    tiles: HashMap<GridCoord, Tile>,
    occupied_squares: Vec<GridCoord>,
    size: GridSize,
    cell_length: f32,
    y_offset: f32,
    x_offset: f32,
}

impl Grid {
    pub fn new(size: GridSize, cell_length: f32, x_center: f32, y_center: f32) -> Self {
        let width = size.width;
        let height = size.height;

        let width_pixels = width as f32 * cell_length;

        let x_offset = -(width_pixels / 2.0);
        let y_offset = y_center;

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
            occupied_squares: Vec::new(),
        }
    }

    pub fn spawn_tower(
        &mut self,
        grid_coord: &GridCoord,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) -> Result<(), TowerSpawnErr> {
        if self.occupied_squares.contains(grid_coord) {
            return Err(TowerSpawnErr::Occupied);
        }

        // retrieve the tile currently at the grid_coord and get its spawn position
        let old_tile = self.get(grid_coord).unwrap();
        let spawn_position = old_tile.get_spawn_position();

        // create a new tower for the Tile
        let new_tower = TowerEntity::Tower(Tower::new(100.0, 1.0, 1.0, 100.0));

        // create the new tile
        let mut new_tile = Tile::new(spawn_position);
        // put the new_tower in the tile
        new_tile.set_entity(new_tower.clone());

        // spawn the entity on the screen
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: spawn_position.x,
                        y: spawn_position.y,
                        z: 0.0,
                    },
                    rotation: Quat::from_rotation_x(0.0),
                    scale: Vec3 {
                        x: self.cell_length / 100.0,
                        y: self.cell_length / 100.0,
                        z: 500.0,
                    },
                },
                texture: asset_server.load("test_tower.png"),
                ..default()
            },
            new_tower,
        ));
        // insert the tile into the Grid's HashMap
        self.insert(*grid_coord, new_tile);

        // add the coordinate to the occupied squares
        self.occupied_squares.push(*grid_coord);

        Ok(())
    }

    pub fn occupied_squares_iter(&self) -> Iter<'_, GridCoord> {
        self.occupied_squares.iter()
    }

    pub fn occupied_squares_iter_mut(&mut self) -> IterMut<'_, GridCoord> {
        self.occupied_squares.iter_mut()
    }

    // getters

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

    // ==========================================================================================================

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

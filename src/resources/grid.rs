use std::collections::HashMap;

use bevy::prelude::*;

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
                grid_map.insert(GridCoord(row, column), Tile);
            });
        });

        Self(grid_map)
    }
}

pub struct Tile;

#[derive(Hash, PartialEq, Eq)]
pub struct GridCoord(usize, usize);

pub struct GridSize {
    pub width: usize,
    pub height: usize,
}

# Grid

```Rust
#[derive(Resource, Debug)]
pub struct Grid {
    tiles: HashMap<GridCoord, Tile>,
    size: GridSize,
    cell_length: f32,
    y_offset: f32,
    x_offset: f32,
}
```

The `Grid struct` represents the tiles in the game map. Each `Tile` has an `Option<TowerEntity>`, i.e it can have a `TowerEntity` or simply be an empty `Tile`.

## Using `Grid`

The `Grid struct` is a Bevy `Resource`, therefore it can be accessed via any Bevy "system" in the following manner.

```Rust
// immutable access
fn system(grid: Res<Grid>) {
    /* function body */
}

// mutable access
fn system(mut grid: ResMut<Grid>) {
    /* function body */
}
```

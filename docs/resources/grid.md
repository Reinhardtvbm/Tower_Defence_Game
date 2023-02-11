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

### Accessing Grid Tiles

To access a `Tile` only

```Rust
if let Some(tile) = grid.get(GridCoord {x: 0, y: 0}) {
    /* Do something with the tile */
}
```

to access the `GridCoord`, `Tile` pair (note 2 parenthesis)

```Rust
if let Some((grid_coord, tile)) = grid.get_key_value(GridCoord {x: 0, y: 0}) {
    /* Do something with the (grid_coord, tile) */
}
```

### Spawing a `Sprite` on the `Grid`

Get the tile and its spawn position

```Rust
let tile = grid.get(/* Some GridCoord */);
let spawn_position = tile.get_spawn_position();
```

Then

### Acessing struct fields

There is a `get_x()` function for each field of `Grid` to get immutable access to the field.

```

```

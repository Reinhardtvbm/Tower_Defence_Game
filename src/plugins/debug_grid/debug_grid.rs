use crate::resources::grid::Grid;
use bevy::prelude::*;

pub struct DebugGridPlugin;

impl Plugin for DebugGridPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(debug_grid)
            .insert_resource(PrevBlock(None))
            .add_system(highlight_cell);
    }
}

#[derive(Resource)]
pub struct PrevBlock(pub Option<Entity>);

impl PrevBlock {
    pub fn fill(&mut self, entity: Entity) {
        self.0 = Some(entity);
    }

    pub fn entity(&self) -> Option<Entity> {
        self.0
    }

    pub fn clear(&mut self) {
        self.0 = None;
    }
}

fn debug_grid(grid: Res<Grid>, mut commands: Commands) {
    // extract grid variables
    let x_offset = grid.get_x_offset();
    let y_offset = grid.get_y_offset();

    let vertical_lines = grid.get_width() + 1;
    let horizontal_lines = grid.get_height() + 1;

    let cell_length = grid.get_cell_length();

    let grid_width = grid.get_width() as f32 * cell_length;
    let grid_height = grid.get_height() as f32 * cell_length;

    // draw the vertical lines of the grid
    (0..vertical_lines).into_iter().for_each(|vert_line| {
        // x position of the line
        let line_pos_x = vert_line as f32 * cell_length;

        // spawn the a line
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(1.0, grid_height)),
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: line_pos_x + x_offset,
                    y: y_offset,
                    z: 0.0,
                },
                ..default()
            },
            ..default()
        });
    });

    // draw the horizontal lines in the grid
    (0..horizontal_lines).into_iter().for_each(|hori_line| {
        // the y position of the line
        let line_pos_y = hori_line as f32 * cell_length - (grid_height / 2.0);

        // draw the line
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(grid_width, 1.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: (grid_width / 2.0) + x_offset,
                    y: line_pos_y + y_offset,
                    z: 0.0,
                },
                ..default()
            },
            ..default()
        });
    })
}

fn highlight_cell(
    grid: Res<Grid>,
    mut commands: Commands,
    windows: Res<Windows>,
    mut prev_block: ResMut<PrevBlock>,
) {
    // Games typically only have one window (the primary window).
    // For multi-window applications, you need to use a specific window ID here.
    let window = windows.get_primary().unwrap();

    // using half the window width and height in later calculations
    let half_window_width = window.width() / 2.0;
    let hald_window_height = window.height() / 2.0;

    // if the cursor is in the window, then proceed
    if let Some(mouse_pos) = window.cursor_position() {
        // if the cursor position maps to a grid coordinate then highlight that cell
        if let Some((grid_coord, tile)) = grid.get_cell_mouse_pos(
            mouse_pos.x - half_window_width,
            mouse_pos.y - hald_window_height,
        ) {
            // print the mouse's adjusted position (mouse coords are different to sprite coords) and
            // the cell it maps to
            println!(
                "mouse ({}, {}) in cell: ({}, {})",
                mouse_pos.x - half_window_width,
                mouse_pos.y - hald_window_height,
                grid_coord.0,
                grid_coord.1
            );

            // remove the highlights associated with previous mouse positions
            if let Some(entity) = prev_block.entity() {
                commands.entity(entity).despawn();
            }

            // clear the Vec that holds those previous highlight entities
            prev_block.clear();

            let position = tile.get_spawn_position();
            let x = position.x;
            let y = position.y;

            // spawn the block that highlights the cell
            let new_entity = commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        custom_size: Some(Vec2::new(
                            grid.get_cell_length(),
                            grid.get_cell_length(),
                        )),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3 { x, y, z: 0.0 },
                        ..default()
                    },
                    ..default()
                })
                .id();

            // put the block in the entities Vec so we can despawn it later
            prev_block.fill(new_entity);
        }
    }
}

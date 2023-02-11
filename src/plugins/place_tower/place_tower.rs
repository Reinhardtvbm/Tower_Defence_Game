use bevy::prelude::*;

use crate::{plugins::debug_grid::debug_grid::PrevBlock, resources::grid::Grid};

use super::place_fsm::{PlaceFSM, PlaceState};

pub struct PlaceTowerPlugin;

impl Plugin for PlaceTowerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PrevBlock(None))
            .insert_resource(PlaceFSM::new())
            .add_system(select_entity_square);
    }
}

fn select_entity_square(
    mut grid: ResMut<Grid>,
    mut commands: Commands,
    mut prev_block: ResMut<PrevBlock>,
    mut state: ResMut<PlaceFSM>,
    windows: Res<Windows>,
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    match state.get_state() {
        PlaceState::Place => {
            // Games typically only have one window (the primary window).
            // For multi-window applications, you need to use a specific window ID here.
            let window = windows.get_primary().unwrap();

            // using half the window width and height in later calculations
            let half_window_width = window.width() / 2.0;
            let half_window_height = window.height() / 2.0;

            // if the cursor is in the window, then proceed
            if let Some(mouse_pos) = window.cursor_position() {
                // if the cursor position maps to a grid coordinate then highlight that cell
                if let Some((grid_coord, tile)) = grid.get_cell_mouse_pos(
                    mouse_pos.x - half_window_width,
                    mouse_pos.y - half_window_height,
                ) {
                    let spawn_position = tile.get_spawn_position();
                    let coord = *grid_coord;

                    if buttons.just_pressed(MouseButton::Left) {
                        grid.spawn_tower(&coord, commands, asset_server);

                        state.toggle();
                    } else {
                        // remove the highlights associated with previous mouse positions
                        if let Some(entity) = prev_block.entity() {
                            commands.entity(entity).despawn();
                        }

                        // clear the Vec that holds those previous highlight entities
                        prev_block.clear();

                        // spawn the entity in the block the mouse if hovering over
                        let new_entity = spawn_sprite(
                            commands,
                            asset_server,
                            spawn_position,
                            grid.get_cell_length(),
                        );

                        // put the block in the entities Vec so we can despawn it later
                        prev_block.fill(new_entity);
                    }
                }
            }
        }
        PlaceState::Idle => {
            if keys.just_pressed(KeyCode::P) {
                state.toggle();
            }
        }
    }
}

fn spawn_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    position: Vec2,
    cell_length: f32,
) -> Entity {
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: position.x,
                    y: position.y,
                    z: 0.0,
                },
                rotation: Quat::from_rotation_x(0.0),
                scale: Vec3 {
                    x: cell_length / 100.0,
                    y: cell_length / 100.0,
                    z: 500.0,
                },
            },
            texture: asset_server.load("test_tower.png"),
            ..default()
        })
        .id()
}

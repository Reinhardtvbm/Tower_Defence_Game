use bevy::prelude::*;
use std::slice::Iter;
use tower_defence::{
    plugins::debug_grid::debug_grid::DebugGridPlugin,
    resources::grid::{Grid, GridSize},
};
//use tower_defence::gui::main_menu::setup_main_menu;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_startup_system(setup_main_menu)
        .add_startup_system(setup)
        .insert_resource(Grid::new(
            GridSize {
                width: 25,
                height: 25,
            },
            25.0,
            -450.0,
            0.0,
        ))
        .add_plugin(DebugGridPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera so we can look at things, even GUI????
    commands.spawn(Camera2dBundle::default());
}

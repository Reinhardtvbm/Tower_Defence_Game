use bevy::prelude::*;

use clap::Parser;
use tower_defence::{
    plugins::debug_grid::debug_grid::DebugGridPlugin,
    resources::grid::{Grid, GridSize},
};
//use tower_defence::gui::main_menu::setup_main_menu;

#[derive(Parser, Debug)]
struct Args {
    /// whether the game should be run in debug
    #[arg(short, long)]
    debug: bool,

    /// whether to launch the map maker instead of the game
    #[arg(short, long)]
    map: bool,
}

fn main() {
    let args = Args::parse();

    // craeting app
    let mut app = App::new();

    if args.debug {
        app.add_plugin(DebugGridPlugin);
    }

    // adding stuff that is always necessary
    app.add_plugins(DefaultPlugins)
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
        .run();
}

fn setup(mut commands: Commands) {
    // Camera so we can look at things, even GUI????
    commands.spawn(Camera2dBundle::default());
}

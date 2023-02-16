use bevy::{prelude::*, reflect::erased_serde::__private::serde::__private::de};

use clap::Parser;
use tower_defence::{
    plugins::{
        debug_grid::debug_grid::DebugGridPlugin, money_plugin::money::MoneyPlugin,
        place_tower::place_tower::PlaceTowerPlugin,
        shoot_tower_plugin::shoot_tower::ShootTowerPlugin,
        spawn_enemy_plugin::spawn_enemy::SpawnEnemyPlugin,
    },
    resources::grid_map::{grid::Grid, grid_size::GridSize},
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
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            mode: WindowMode::BorderlessFullscreen,
            title: "RhinoTD".into(),
            cursor_grab_mode: bevy::window::CursorGrabMode::None,
            ..default()
            
        },
        ..default()
    }))
    //.add_startup_system(setup_main_menu)
    .add_startup_system(setup)
    .insert_resource(Grid::new(
        GridSize {
            width: 33,
            height: 17,
        },
        40.0,
        -700.0,
        0.0,
    ))
    .add_plugin(PlaceTowerPlugin)
    .add_plugin(SpawnEnemyPlugin)
    .add_plugin(ShootTowerPlugin)
    .add_plugin(MoneyPlugin)
    .run();
}

fn setup(mut commands: Commands) {
    // Camera so we can look at things, even GUI????
    commands.spawn(Camera2dBundle::default());
}

use bevy::prelude::*;

use tower_defence::plugins::debug_grid::debug_grid::DebugGridPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_startup_system(setup_main_menu)
        .add_startup_system(setup)
        .add_plugin(DebugGridPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera so we can look at things, even GUI????
    commands.spawn(Camera2dBundle::default());
}

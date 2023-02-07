use bevy::prelude::*;
use tower_defence::gui::main_menu::setup_main_menu;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_main_menu)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera so we can look at things, even GUI????
    commands.spawn(Camera2dBundle::default());
}
